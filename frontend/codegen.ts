import type { CodegenConfig } from "@graphql-codegen/cli";

// 起動中の BE から introspection でスキーマを取得する。
// codegen を実行する前に `cd backend && cargo make run` で BE を起動しておくこと。
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
        // 未指定だと v4 形式のフックが生成される
        reactQueryVersion: 5,
        // 未指定だと TError が unknown になり error.message が型エラーになる
        // fetcher / gqlRequest は Error を throw する
        errorType: "Error",
        // 未指定だと graphql-tag の DocumentNode が渡され、fetch ベースの fetcher と噛み合わない
        documentMode: "string",
        fetcher: "@/app/lib/graphql-client#fetcher",
      },
    },
  },
};

export default config;
