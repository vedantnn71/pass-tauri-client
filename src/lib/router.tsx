import {
  Outlet,
  Router,
  Route,
  RootRoute,
} from '@tanstack/router'
import { App } from "~/app";
import { Nav } from "~/components/nav";
import { Toaster } from "react-hot-toast";

import { QueryClientProvider, QueryClient } from '@tanstack/react-query';

const queryClient = new QueryClient()

const rootRoute = new RootRoute({
  component: Root,
})

export const indexRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/',
  component: App,
})

export const routeTree = rootRoute.addChildren([indexRoute])

export const router = new Router({ routeTree, defaultPreload: 'intent' })

declare module '@tanstack/router' {
  interface Register {
    router: typeof router
  }
}

function Root() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="min-w-screen h-screen flex flex-col select-none">
        <Nav />
        <Outlet />
        <Toaster />
      </div>
    </QueryClientProvider>
  )
}

