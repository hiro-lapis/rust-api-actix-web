# rust-api-actix-web: GraphQL スキーマ生成 → フロント型共有(codegen)

## Context

現状、フロント(`frontend/app/lib/queries.ts`)は GraphQL クエリ文字列も型(`Dashboard`)も**手書き**で、バックエンド(`src/bin/main.rs` の async-graphql スキーマ)と二重管理になっている。スキーマ変更時に型がズレる。

バックエンドを Single Source of Truth とし、**Rust から SDL を生成 → フロントが codegen で TypeScript 型+react-query フックを自動生成**するパイプラインを構築する。構成は既存の社内プロジェクト mogok のパターン（同一スタック: actix-web + async-graphql + Next.js + react-query + npm）に倣う。**mogok のコード・型・機密は一切コピーせず、仕組み(専用bin / 共有 `/schema` / 3層codegen / 差分チェック)のみを移植する。**

## 決定事項（ユーザー確認済み）

- SDL 出力: **lib化 + 専用bin**（mogok の `mogok-server-gen` 相当を、本プロジェクトでは同一パッケージの lib + 2バイナリで実現）
- codegen 範囲: **mogok に倣う** = `typescript` + `typescript-operations` + `typescript-react-query` の3層（フックまで生成 + カスタム fetcher）
- 再生成フロー: **pre-commit 自動化**（既存 `.githooks/pre-commit` の「差分検知→再生成→`git add`」パターンを踏襲）

## mogok から移植する構成（抽象パターンのみ）

| 観点 | mogok の方式 | 本プロジェクトへの適用 |
|---|---|---|
| SDL 出力 | 専用bin が lib の `Schema::finish().sdl()` を呼ぶ | 同一パッケージに `src/lib.rs` + `src/bin/schema.rs` |
| SDL 配置 | repo直下 `/schema/*.graphql` | repo直下 `schema/schema.graphql` |
| フロント参照 | `codegen.yaml` が `../schema/schema.graphql` | `frontend/codegen.ts` が `../schema/schema.graphql` |
| codegen | typescript + operations + react-query + custom fetcher | 同左（ただし **react-query v5 対応** が必要） |
| 生成物 | `src/gen/graphql.ts` | `frontend/gen/graphql.ts`（import `@/gen/graphql`） |
| 差分強制 | CI で `git diff --exit-code` | 既存 `.githooks/pre-commit` に組込 + `git add` |

> **mogok との差分注意**: mogok は react-query **v4**(`@tanstack/react-query@4.36.1`)。本プロジェクトは **v5**(`5.100.10`)。`typescript-react-query` は `reactQueryVersion: 5` 指定が必須（未指定だと v4 形式のフックが生成され動かない）。

## 変更内容

### 1. バックエンド: lib化 + SDL 生成bin

**新規 `src/lib.rs`** — 現在 `src/bin/main.rs` にある `Query` / `Mutation` / `type ApiSchema` / `establish_connection()` をここへ移動して `pub` 化。SDL 生成は resolver 実行不要なので DB 非依存。公開 API を2つ追加:

```rust
pub fn build_schema() -> ApiSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
}
pub fn schema_sdl() -> String {
    build_schema().sdl()
}
```

**`src/bin/main.rs`** — サーバ起動処理だけ残す。スキーマは `app::build_schema()` を使用（CORS 追加済みの `.wrap(Cors...)` はそのまま）。

**新規 `src/bin/schema.rs`** — SDL をファイル出力。

```rust
fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("schema")?;
    std::fs::write("schema/schema.graphql", app::schema_sdl())?;
    Ok(())
}
```

**`Cargo.toml`** — `schema` bin を明示追加（`src/lib.rs` は自動でlibターゲット化）:

```toml
[[bin]]
name = "schema"
path = "src/bin/schema.rs"
```

**`Makefile.toml`** — DB 非依存の生成タスク（`before-build` に依存させない = DBコンテナを立てない）:

```toml
[tasks.gen-schema]
command = "cargo"
args = ["run", "--quiet", "--bin", "schema"]
```

### 2. 共有 SDL ファイル

- `schema/schema.graphql`（repo直下、`gen-schema` が生成）。**コミット対象**（共有アーティファクト兼 codegen 入力）。

### 3. フロント: codegen 導入

**新規 `frontend/codegen.ts`** — mogok の3層構成 + v5対応 + カスタム fetcher:

```ts
import type { CodegenConfig } from "@graphql-codegen/cli";
const config: CodegenConfig = {
  schema: "../schema/schema.graphql",
  documents: ["app/**/*.graphql"],
  generates: {
    "gen/graphql.ts": {
      plugins: ["typescript", "typescript-operations", "typescript-react-query"],
      config: {
        reactQueryVersion: 5,                       // ← v5 必須
        fetcher: "@/app/lib/graphql-client#fetcher", // 既存 fetch 層を再利用
      },
    },
  },
};
export default config;
```

**`frontend/app/lib/graphql-client.ts`** — 既存 `gqlRequest` を活かし、typescript-react-query が要求する fetcher シグネチャ(`(query, variables) => () => Promise<TData>`)の `fetcher` を export 追加。

**新規 `frontend/app/lib/queries/dashboard.graphql`** — 手書きクエリを document 化（codegen が走査）:

```graphql
query Dashboard { now totalPhotos }
```

**`frontend/app/lib/queries.ts`** — 削除。手書き `useDashboard` / `Dashboard` 型は生成物 `useDashboardQuery`(from `@/gen/graphql`)に置換。

**`frontend/app/page.tsx`** — import を `useDashboard`(手書き) → `useDashboardQuery`(生成) に変更。`data.now` / `data.totalPhotos` は生成型で型付け。

**`frontend/package.json`** — devDependencies に codegen 一式を**固定バージョン**で追加（`comment_rule_about_versionning` 準拠）。起点バージョン（mogok 実績、react-query v5 対応の最新に解決）:
`@graphql-codegen/cli` / `@graphql-codegen/typescript` / `@graphql-codegen/typescript-operations` / `@graphql-codegen/typescript-react-query` / `graphql`。
scripts 追加:

```json
"codegen": "graphql-codegen --config codegen.ts",
"schema:update": "cargo make --cwd .. gen-schema && npm run codegen"
```

### 4. pre-commit 自動化

**`.githooks/pre-commit`** — 既存の `.sqlx` ブロック(差分検知→再生成→`git add`)と同じ書式で追記:

- `./src` or `./db-schema` に差分 → `cargo make gen-schema` → `git add schema/schema.graphql`
- `schema/schema.graphql` or `frontend/app/**/*.graphql` に差分 → `cd frontend && npm run codegen` → `git add frontend/gen`

> **トレードオフ**: mogok はこの再生成チェックを**CI(GitHub Actions)**で実施（pre-commit を軽く保つため）。本プロジェクトはユーザー選択に従い pre-commit で自動化するが、Rust コンパイル + npm codegen の分コミットが遅くなる。パス差分ガードで対象変更時のみ実行し軽量化する。将来 CI へ移す選択肢も残す。

## 対象ファイル

- 新規: `src/lib.rs`, `src/bin/schema.rs`, `frontend/codegen.ts`, `frontend/app/lib/queries/dashboard.graphql`, `schema/schema.graphql`(生成)
- 変更: `src/bin/main.rs`(slim化), `Cargo.toml`(bin追加), `Makefile.toml`(gen-schema), `frontend/app/lib/graphql-client.ts`(fetcher export), `frontend/app/page.tsx`(生成フック使用), `frontend/package.json`(deps+scripts), `.githooks/pre-commit`
- 削除: `frontend/app/lib/queries.ts`

## 検証（end-to-end）

1. `cargo build` が通る（lib化 + 2バイナリ構成でコンパイル成功）。
2. `cargo make gen-schema` → `schema/schema.graphql` が生成され、`now` / `totalPhotos` / `users` / `categories` / `createUser` 等が SDL に含まれることを確認。DB 未起動でも成功すること。
3. `cd frontend && npm install && npm run codegen` → `frontend/gen/graphql.ts` に `useDashboardQuery` と `Dashboard` 系型が生成される。
4. `npx tsc --noEmit` / `npm run lint` がエラーなし。
5. 両サーバ起動（`cargo make run` + `npm run dev`）→ `http://localhost:3000` で `now`(日本時間)/`totalPhotos`(42) が生成フック経由で表示。DevTools で :8080 への POST が200。
6. pre-commit 検証: `src/lib.rs` の Query に一時的なフィールド追加 → `git add` → コミット時に `schema.graphql` が自動再生成・`git add` されることを確認（確認後 revert）。

## 補足

- 現状の `db-schema/src/model.rs` の GraphQL 型(User/Category/SubCategory)はそのまま SDL に反映される（変更不要）。
- 生成物(`frontend/gen/`)と `schema/schema.graphql` は**コミットする**（gitignore しない）。これが「共有」の実体であり pre-commit/CI 差分チェックの前提。
- GCP デプロイ時も SDL はビルド成果物ではなくリポジトリ管理物なので、デプロイ構成には影響しない。
