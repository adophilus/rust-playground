import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { Toaster } from "sonner";

export const Route = createRootRoute({
  component: RootComponent,
});

const client = new QueryClient();

const Devtools = () => {
  return <TanStackRouterDevtools />;
};

function RootComponent() {
  return (
    <QueryClientProvider client={client}>
      <Outlet />
      <Toaster />
      <Devtools />
    </QueryClientProvider>
  );
}
