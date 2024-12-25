import { Outlet, createRootRoute } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { Toaster } from "sonner";
import { Provider as Web3Provider } from "@/lib/web3";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

export const Route = createRootRoute({
  component: RootComponent,
});

const Devtools = () => {
  return <TanStackRouterDevtools />;
};

const client = new QueryClient();

function RootComponent() {
  return (
    <QueryClientProvider client={client}>
      <Web3Provider>
        <div className="min-h-screen">
          <Outlet />
          <Toaster />
          <Devtools />
        </div>
      </Web3Provider>
    </QueryClientProvider>
  );
}
