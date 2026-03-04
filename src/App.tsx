import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Dashboard } from './components/Dashboard/Dashboard';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchInterval: 1000,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="min-h-screen bg-adwaita-bg">
        <div className="container mx-auto p-8">
          <h1 className="text-3xl font-bold mb-8">Upakor-G14</h1>
          <Dashboard />
        </div>
      </div>
    </QueryClientProvider>
  );
}

export default App;
