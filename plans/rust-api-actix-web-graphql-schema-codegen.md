# rust-api-actix-web: GraphQL スキーマ公開 → FE 型自動生成 (codegen)

作成日: 2026-09-12

## 1. 背景 / 課題

フロントエンド [`frontend/app/lib/queries.ts`](../frontend/app/lib/queries.ts) は GraphQL のクエリ文字列とレスポンス型 `Dashboard` を**手書き**しており、バックエンド [`backend/src/lib.rs`](../backend/src/lib.rs) の async-graphql スキーマと二重管理になっている。BE 側のスキーマを変更しても FE の型は追従せず、ズレに気づけない。

バックエンドを Single Source of Truth とし、**起動中の BE から introspection でスキーマを取得 → FE が graphql-codegen で TypeScript 型 + react-query フックを自動生成**するパイプラインを構築する。

### 現状（調査結果）

| 対象 | 状態 |
|---|---|
| `backend/src/lib.rs` | `Query` / `Mutation` / `ApiSchema` / `build_schema()` が既に `pub`。introspection は async-graphql のデフォルトで有効 |
| `backend/src/bin/main.rs` | `build_schema()` を使わず `Schema::build(...)` を**再実装**している（スキーマ定義が2箇所） |
| `frontend/app/lib/queries.ts` | クエリ文字列・型ともに手書き |
| `frontend/app/lib/graphql-client.ts` | `gqlRequest<T>(query, variables)` あり。エンドポイントは `NEXT_PUBLIC_GRAPHQL_ENDPOINT ?? "http://localhost:8080/"` |
| react-query | `@tanstack/react-query` **5.100.10**（v5 系） |
| パッケージマネージャ | `package-lock.json` = npm 運用。ただし `package.json` の `packageManager` は `pnpm@8.7.0` 表記で**不整合**（§7 参照） |
| codegen 依存 | 未導入 |
| CI | なし（`.github/` 不在）。`.githooks/pre-commit` に「差分検知→再生成→`git add`」パターンあり |

> 旧メモ `.idea/plans/rust-api-actix-web-graphql-schema-codegen.md` は「SDL ファイルをコミットする」方式で書かれており、かつ `backend/` ディレクトリ移動**前**のパスを指している。本プランがそれを**置き換える**。

## 2. 決定事項（ユーザー確認済み）

| 論点 | 決定 |
|---|---|
| スキーマの「公開」方法 | **起動中サーバからの introspection**。SDL ファイルはリポジトリにコミットしない |
| codegen の生成粒度 | **3層**: `typescript` + `typescript-operations` + `typescript-react-query`（react-query フックまで生成） |
| 再生成の契機 | **手動 npm script のみ**（`npm run codegen`）。pre-commit / CI での自動化はしない |
| 生成物の git 管理 | **コミットする**（`frontend/gen/graphql.ts`） |
| クエリ定義の置き場 | **`.graphql` 別ファイル** |

## 3. 方式概要

```
[ BE: cargo make run ]                    ← 先に起動しておく必要がある
        │  http://localhost:8080/  (POST introspection)
        ▼
[ frontend/codegen.ts ]  ── documents: frontend/app/lib/queries/*.graphql
        │
        ▼
[ frontend/gen/graphql.ts ]  ← コミット対象。型 + DocumentString + useXxxQuery
        │  fetcher: @/app/lib/graphql-client#fetcher
        ▼
[ frontend/app/page.tsx ]  useDashboardQuery()
```

## 4. 検証済みの技術的制約

npm レジストリとパッケージ内部の実装を確認して判明した、**設定を誤ると動かない**点。
4-1〜4-4 は実装着手前、4-5 は実装中に判明したもの。

### 4-1. `graphql` は 16 系に固定する

`@graphql-codegen/typescript-react-query@7.0.5` の peerDependencies は `graphql` が `^16.0.0` まで。`graphql@17`（現 latest）は**対象外**。

```
@graphql-codegen/cli@7.4.1                    peer graphql: ... || ^16.0.0 || ^17.0.0
@graphql-codegen/typescript@6.1.0             peer graphql: ... || ^16.0.0 || ^17.0.0
@graphql-codegen/typescript-react-query@7.0.5 peer graphql: ... || ^16.0.0    ← 17 なし
```

→ `graphql` は **`16.14.2`**（`dist-tags.latest-16`）を固定指定する。

### 4-2. `reactQueryVersion: 5` の明示が必須

`typescript-react-query` のデフォルトは **v4**。未指定だと v5 と非互換なフックが生成される。

```js
// node_modules/@graphql-codegen/typescript-react-query/esm/visitor.js
const defaultReactQueryVersion = !rawConfig.reactQueryVersion && rawConfig.legacyMode ? 3 : 4;
```

本プロジェクトは react-query `5.100.10` なので `reactQueryVersion: 5` を指定する。

### 4-3. `documentMode: "string"` が必須

`documentMode` のデフォルトは `graphQLTag`。この場合、生成物は `graphql-tag` の `gql` で `DocumentNode` を作り、それを fetcher に渡す。既存の `gqlRequest` は**クエリ文字列**を受け取る `fetch` ベースなので噛み合わない。

```js
// node_modules/@graphql-codegen/visitor-plugin-common/esm/client-side-base-visitor.js
documentMode: getConfigValue(rawConfig.documentMode, DocumentMode.graphQLTag),
```

→ `documentMode: "string"` を指定する。`graphql-tag` の追加依存は不要になる。

> **実装時の実測**: `documentMode: "string"` が生成するのは素の `string` ではなく、`String` を継承した
> `TypedDocumentString` クラスのインスタンス。実行時は `JSON.stringify` で文字列にシリアライズされるので
> 問題ないが、**型としては `string` に代入できない**。fetcher の引数型を `string` にすると
> `TS2345: Argument of type 'TypedDocumentString<unknown, unknown>' is not assignable to parameter of type 'string'`
> になる（§5-2 で対応済み）。また生成物が `@graphql-typed-document-node/core` を import するため、
> 同パッケージを明示的に devDependencies へ追加する必要がある（従来は codegen の推移的依存でしかなかった）。

### 4-4. カスタム fetcher のシグネチャ

`fetcher-custom-mapper.js` を読んで確認した、生成コードが期待する形：

```
// query hook 内:   fetcher<TData, TVariables>(DocumentString, variables)  → () => Promise<TData> を queryFn に渡す
// mutation hook 内: (variables) => fetcher<TData, TVariables>(DocumentString, variables)()
// use...Query.fetcher = (variables, options?: RequestInit['headers']) => fetcher(Doc, variables, options)
```

→ fetcher は `(query, variables?, options?) => () => Promise<TData>`（**サンクを返す**）である必要がある。

### 4-5. `errorType: "Error"` の指定

`typescript-react-query` の `errorType` のデフォルトは `unknown`。未指定だと生成フックが
`TError = unknown` になり、既存の `page.tsx` の `error.message` が
`Type error: 'error' is of type 'unknown'.` で落ちる。

`gqlRequest` / `fetcher` は `new Error(...)` を throw するので `errorType: "Error"` を指定する。

> `npx tsc --noEmit` と `next build` の TypeScript チェックは**同じ結果になる**が、本件は
> page.tsx を差し替える前に tsc を流していたため `next build` で初めて顕在化した。検証順序に注意。

## 5. 変更内容

### 5-1. バックエンド

**`backend/src/bin/main.rs`** — スキーマ構築の二重定義を解消。

```rust
// before
use app::{Query, Mutation, ApiSchema};
let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

// after
use app::{ApiSchema, build_schema};
let schema = build_schema();
```

不要になる `use async_graphql::{EmptySubscription, Schema};` と `use app::{Query, Mutation}` を削除する。

> introspection が公開するスキーマと `build_schema()` を一致させるため。現状は同一内容だが、片方だけ変更されると FE の生成型が実サーバとズレる。

**`backend/src/lib.rs`** — スキーマ形状の回帰テストを追加（TDD: 先に落ちるテストを書く）。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_exposes_dashboard_fields() {
        let sdl = build_schema().sdl();
        assert!(sdl.contains("now: String!"));
        assert!(sdl.contains("totalPhotos: Int!"));
    }
}
```

DB 非依存（resolver を実行しないため）。`cargo make test` で実行できる。

### 5-2. フロントエンド

**新規 `frontend/codegen.ts`**

```ts
import type { CodegenConfig } from "@graphql-codegen/cli";

const SCHEMA_URL = process.env.GRAPHQL_SCHEMA_URL ?? "http://localhost:8080/";

const config: CodegenConfig = {
  schema: SCHEMA_URL,
  documents: ["app/**/*.graphql"],
  generates: {
    "gen/graphql.ts": {
      plugins: [
        "typescript",
        "typescript-operations",
        "typescript-react-query",
      ],
      config: {
        reactQueryVersion: 5,                          // §4-2
        errorType: "Error",                            // §4-5
        documentMode: "string",                        // §4-3
        fetcher: "@/app/lib/graphql-client#fetcher",   // §4-4
      },
    },
  },
};

export default config;
```

**新規 `frontend/app/lib/queries/dashboard.graphql`**

```graphql
query Dashboard {
  now
  totalPhotos
}
```

**`frontend/app/lib/graphql-client.ts`** — `gqlRequest` に任意ヘッダを受け取れるようにし、`fetcher` を追加 export。

```ts
export async function gqlRequest<T>(
  query: string,
  variables?: Record<string, unknown>,
  headers?: RequestInit["headers"],
): Promise<T> {
  // RequestInit["headers"] は Record / Headers / string[][] のいずれも取り得るため、
  // スプレッドではなく Headers 経由でマージする
  const merged = new Headers({ "Content-Type": "application/json" });
  new Headers(headers).forEach((value, key) => merged.set(key, value));

  const res = await fetch(ENDPOINT, {
    method: "POST",
    headers: merged,
    body: JSON.stringify({ query, variables }),
  });
  // ...以降は既存のまま
}

// documentMode: "string" の生成物は String を継承した TypedDocumentString なので、
// プリミティブな string と両方を受け取れるようにする (§4-3)
type GraphQLDocument = string | { toString(): string };

export function fetcher<TData, TVariables>(
  query: GraphQLDocument,
  variables?: TVariables,
  options?: RequestInit["headers"],
): () => Promise<TData> {
  return () =>
    gqlRequest<TData>(
      query.toString(),
      variables as Record<string, unknown> | undefined,
      options,
    );
}
```

**`frontend/app/page.tsx`** — 生成フックに差し替え。

```ts
// before
import { useDashboard } from "@/app/lib/queries";
const { data, isPending, isError, error } = useDashboard()

// after
import { useDashboardQuery } from "@/gen/graphql";
const { data, isPending, isError, error } = useDashboardQuery()
```

`data.now` / `data.totalPhotos` は生成型 `DashboardQuery` で型付けされる（`now: string` / `totalPhotos: number`）。JSX 側の変更は不要。

**`frontend/app/lib/queries.ts`** — **削除**。

**`frontend/package.json`** — devDependencies をバージョン固定で追加（`comment_rule_about_versionning` 準拠）。

```json
"devDependencies": {
  "@graphql-codegen/cli": "7.4.1",
  "@graphql-codegen/typescript": "6.1.0",
  "@graphql-codegen/typescript-operations": "6.1.6",
  "@graphql-codegen/typescript-react-query": "7.0.5",
  "@graphql-typed-document-node/core": "3.2.0",
  "graphql": "16.14.2"
}
```

scripts に追加：

```json
"codegen": "graphql-codegen --config codegen.ts"
```

**`frontend/eslint.config.mjs`** — 生成物を lint 対象外にする。

```js
globalIgnores([
  ".next/**",
  "out/**",
  "build/**",
  "next-env.d.ts",
  "gen/**",        // ← 追加: graphql-codegen 生成物
]),
```

**`frontend/tsconfig.json`** — 変更不要。既存の `paths: { "@/*": ["./*"] }` で `@/gen/graphql` が解決される。

**`frontend/.gitignore`** — 変更不要。`gen/` は無視対象に入っていないため、そのままコミットされる。

### 5-3. ドキュメント

**`README.md`** — FE セクションに codegen 手順を追記。

```
## FE

cd frontend
npm install
npm run dev

### GraphQL 型の再生成

BE を起動した状態で実行する（introspection でスキーマを取得するため）。

cargo make run          # 別ターミナルで BE 起動
cd frontend && npm run codegen
```

## 6. 検証手順

| # | 手順 | 期待結果 |
|---|---|---|
| 1 | `cd backend && cargo make test` | `schema_exposes_dashboard_fields` が通る（DB 未起動でも可） |
| 2 | `cd backend && cargo make run` | `http://localhost:8080` で playground が開く |
| 3 | `cd frontend && npm install` | peer 警告なしでインストール完了（graphql 16 固定が効いていること） |
| 4 | `cd frontend && npm run codegen` | `frontend/gen/graphql.ts` が生成され、`useDashboardQuery` / `DashboardQuery` / `DashboardDocument`（string）が含まれる |
| 5 | 生成物に `graphql-tag` の import が**ない**ことを確認 | `documentMode: "string"` が効いている |
| 6 | `cd frontend && npx tsc --noEmit` | エラーなし |
| 7 | `cd frontend && npm run lint` | エラーなし |
| 8 | BE + `npm run dev` 起動 → `http://localhost:3000` | `now`(日本時間) と `totalPhotos`(42) が生成フック経由で表示される。DevTools で :8080 への POST が 200 |
| 9 | BE を停止して `npm run codegen` | 接続エラーで失敗する（= introspection 方式の制約が想定通り顕在化する） |
| 10 | `backend/src/lib.rs` の `Query` に一時フィールド追加 → BE 再起動 → `npm run codegen` | 生成物に新フィールドの型が現れる。確認後 revert |

## 7. トレードオフと既知の注意点

- **codegen に BE 起動が必須**。clone 直後や BE を落としている状態では `npm run codegen` が失敗する。生成物をコミットする決定により、FE 単体での `tsc` / `lint` / `build` は通る状態を保てる。
- **スキーマ変更の追従は手動**。BE のスキーマを変えても `npm run codegen` を忘れると生成型が古いままコミットされる。将来的に CI で「BE を起動 → codegen → `git diff --exit-code`」を回す余地を残す（本プランの対象外）。
- **`package.json` の `packageManager` 不整合**（`pnpm@8.7.0` と書かれているが `package-lock.json` = npm 運用）。corepack 有効環境では `npm run codegen` が意図せず pnpm 経由になり得る。本プランでは**触らず据え置き**とし、別タスクとして切り出す。
- `graphql` を 16 系に固定するため、将来 `graphql@17` に上げる際は `typescript-react-query` の peerDeps 更新待ちになる。

## 8. 対象外（やらないこと）

- SDL ファイル（`schema/schema.graphql`）の生成・コミット — introspection 方式を選択したため不要
- pre-commit / CI での自動再生成
- 外部への配布（npm パッケージ化・GitHub Release）
- `users` / `categories` / `createUser` など Dashboard 以外のクエリの FE 実装 — スキーマ全体の型は生成されるが、document 化は必要になった時点で行う

## 9. 対象ファイル一覧

- 新規: `frontend/codegen.ts`, `frontend/app/lib/queries/dashboard.graphql`, `frontend/gen/graphql.ts`(生成)
- 変更: `backend/src/bin/main.rs`, `backend/src/lib.rs`(テスト追加), `frontend/app/lib/graphql-client.ts`, `frontend/app/page.tsx`, `frontend/package.json`, `frontend/eslint.config.mjs`, `README.md`
- 削除: `frontend/app/lib/queries.ts`

## 10. 実装結果

### 10-1. プランからの差分

| 差分 | 理由 |
|---|---|
| `codegen.ts` に `errorType: "Error"` を追加 | デフォルトの `TError = unknown` だと `page.tsx` の `error.message` が型エラー（§4-5） |
| `fetcher` の引数型を `string` → `string \| { toString(): string }` | `documentMode: "string"` の生成物が `TypedDocumentString`（`String` サブクラス）で `string` に代入不可（§4-3） |
| devDependencies に `@graphql-typed-document-node/core@3.2.0` を追加 | 生成物が直接 import するため。従来は codegen の推移的依存で解決されていただけで、明示宣言がないと壊れ得る |

### 10-2. 検証実績

| # | 検証 | 結果 |
|---|---|---|
| 1 | `cargo test --lib` | ✅ `schema_exposes_dashboard_fields` pass。アサーションを意図的に壊すと FAIL することも確認済み |
| 2 | `cargo build` / `cargo clippy --all-targets` / `cargo fmt --check` | ✅ 警告・エラーなし |
| 3 | BE 起動 → `POST /` introspection | ✅ `__schema.types` に `Query` / `Mutation` / `User` / `Category` / `Renamed` / `CreateUserInput` / `NaiveDate` を確認 |
| 4 | `npm run codegen` | ✅ `frontend/gen/graphql.ts` 生成。`useDashboardQuery` / `DashboardQuery` / `DashboardDocument` を含む |
| 5 | 生成物に `graphql-tag` の import なし | ✅ `documentMode: "string"` が効いている |
| 6 | `npx tsc --noEmit` | ✅ エラーなし |
| 7 | `npm run lint` | ✅ エラーなし |
| 8 | `npm run build` | ✅ 成功（`/` が static prerender） |
| 9 | BE + `npm run dev` → `http://localhost:3000` | ✅ `now: 2026年9月12日 13:6:38` / `totalPhotos: 42` を生成フック経由で表示。`OPTIONS http://localhost:8080/ → 200` / `POST http://localhost:8080/ → 200` |
| 10 | `GRAPHQL_SCHEMA_URL=http://localhost:9999/ npm run codegen` | ✅ `ECONNREFUSED` で明示的に失敗し、既存の生成物を壊さない。env による上書きも動作 |

### 10-3. 積み残し（本 PR では対応しない）

- **`npm install` 時の EBADENGINE 警告**: `@graphql-codegen/cli@7.4.1` の推移的依存 `yargs@18.1.0` が Node `^20.19.0 || ^22.12.0 || >=23` を要求するのに対し、ローカルは `v20.18.0`。警告のみで codegen は正常動作するが、Node を 20.19+ に上げると解消する。
- **`npm audit` の脆弱性**: `--omit=dev` で残るのは `sharp`（Next.js の依存）由来のみで、本 PR で追加した依存とは無関係。既存の課題。
- **`next build` の workspace root 警告**: `/Users/lapis/package-lock.json` を root と誤検出している（ローカル環境固有）。本 PR とは無関係。
- **`package.json` の `packageManager` 不整合**（§7）: 据え置き。
