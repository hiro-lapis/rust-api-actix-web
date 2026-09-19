const ENDPOINT =
  process.env.NEXT_PUBLIC_GRAPHQL_ENDPOINT ?? "http://localhost:8080/";

type GraphQLResponse<T> = {
  data?: T;
  errors?: { message: string }[];
};

async function gqlRequest<T>(
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

  if (!res.ok) {
    throw new Error(`GraphQL HTTP ${res.status}`);
  }

  const json = (await res.json()) as GraphQLResponse<T>;
  if (json.errors?.length) {
    throw new Error(json.errors[0].message);
  }
  if (json.data === undefined) {
    throw new Error("GraphQL response has no data");
  }
  return json.data;
}

// documentMode: "string" の生成物は String を継承した TypedDocumentString なので、
// プリミティブな string と両方を受け取れるようにする
type GraphQLDocument = string | { toString(): string };

// graphql-codegen の typescript-react-query が呼び出す fetcher。
// 生成コードは戻り値のサンクを queryFn に渡すため、Promise ではなく関数を返す。
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
