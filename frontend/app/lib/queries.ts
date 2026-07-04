import { useQuery } from "@tanstack/react-query";
import { gqlRequest } from "@/app/lib/graphql-client";

type Dashboard = {
  now: string;
  totalPhotos: number;
};

const DASHBOARD_QUERY = `
  query Dashboard {
    now
    totalPhotos
  }
`;

export function useDashboard() {
  return useQuery({
    queryKey: ["dashboard"],
    queryFn: () => gqlRequest<Dashboard>(DASHBOARD_QUERY),
  });
}
