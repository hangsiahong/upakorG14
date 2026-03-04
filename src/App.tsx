import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useState } from 'react';
import { Sidebar } from './components/Sidebar/Sidebar';
import { Dashboard } from './components/Dashboard/Dashboard';
import { GpuControl } from './components/GpuControl/GpuControl';
import { Settings } from './components/Settings/Settings';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchInterval: 2000, // Poll every 2 seconds
    },
  },
});

function App() {
  const [currentPage, setCurrentPage] = useState('dashboard');

  const renderPage = () => {
    switch (currentPage) {
      case 'dashboard':
        return <Dashboard />;
      case 'power':
        return <Dashboard />; // Reuse dashboard for now
      case 'gpu':
        return <GpuControl />;
      case 'battery':
        return <Dashboard />; // Reuse dashboard for now
      case 'settings':
        return <Settings />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <QueryClientProvider client={queryClient}>
      <div className="flex min-h-screen bg-gray-100">
        <Sidebar currentPage={currentPage} onPageChange={setCurrentPage} />
        <main className="flex-1 p-8 overflow-auto">
          <div className="max-w-6xl">
            <h1 className="text-3xl font-bold mb-6 text-gray-900">
              {currentPage.charAt(0).toUpperCase() + currentPage.slice(1)}
            </h1>
            {renderPage()}
          </div>
        </main>
      </div>
    </QueryClientProvider>
  );
}

export default App;
