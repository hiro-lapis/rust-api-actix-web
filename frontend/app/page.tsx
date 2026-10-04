"use client";

import { useState } from "react";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { useDashboardQuery } from "@/gen/graphql";

export default function App() {
  // in order to manage cache properly, we need to use useState and keep the same instance
  // https://tanstack.com/query/v5/docs/eslint/stable-query-client
  const [stableQueryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            refetchOnWindowFocus: false,
            retry: false,
          },
        },
      }),
  );
  return (
    // Provide the client to your App
    <QueryClientProvider client={stableQueryClient}>
      <Home />
    </QueryClientProvider>
  );
}

function Home() {
  const { data, isPending, isError, error } = useDashboardQuery();

  return (
    <div className="flex flex-col flex-1 items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex flex-1 w-full max-w-3xl flex-col items-start justify-center gap-6 py-32 px-16 bg-white dark:bg-black">
        <h1 className="text-3xl font-semibold tracking-tight text-black dark:text-zinc-50">
          GraphQL API dashboard
        </h1>

        {isPending && (
          <p className="text-lg text-zinc-600 dark:text-zinc-400">Loading...</p>
        )}

        {isError && (
          <p className="text-lg text-red-600 dark:text-red-400">
            Error: {error.message}
          </p>
        )}

        {data && (
          <dl className="flex flex-col gap-4 text-lg">
            <div className="flex gap-2">
              <dt className="font-medium text-zinc-600 dark:text-zinc-400">
                now:
              </dt>
              <dd className="text-black dark:text-zinc-50">{data.now}</dd>
            </div>
            <div className="flex gap-2">
              <dt className="font-medium text-zinc-600 dark:text-zinc-400">
                totalPhotos:
              </dt>
              <dd className="text-black dark:text-zinc-50">
                {data.totalPhotos}
              </dd>
            </div>
          </dl>
        )}
      </main>
    </div>
  );
}
