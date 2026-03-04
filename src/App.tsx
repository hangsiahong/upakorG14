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

  const getPageTitle = () => {
    const titles: Record<string, string> = {
      dashboard: 'SYSTEM DASHBOARD',
      power: 'POWER PROFILES',
      gpu: 'GPU CONTROL',
      battery: 'BATTERY STATUS',
      settings: 'SYSTEM SETTINGS',
    };
    return titles[currentPage] || currentPage.toUpperCase();
  };

  return (
    <QueryClientProvider client={queryClient}>
      <div className="flex min-h-screen bg-upakor-bg">
        {/* Sidebar - Fixed */}
        <Sidebar currentPage={currentPage} onPageChange={setCurrentPage} />

        {/* Main Content - Scrollable */}
        <main className="flex-1 overflow-auto relative z-10">
          <div className="max-w-7xl mx-auto p-8">
            {/* Page Header */}
            <div className="mb-8 animate-fade-in">
              <div className="flex items-center gap-3 mb-2">
                <div className="h-8 w-1 bg-upakor-accent" />
                <h1 className="font-display text-3xl font-bold tracking-tight">
                  {getPageTitle()}
                </h1>
              </div>
              <p className="font-body text-sm text-upakor-fg-muted ml-4">
                {currentPage === 'dashboard' && 'Real-time hardware monitoring and control'}
                {currentPage === 'gpu' && 'Graphics processing unit mode management'}
                {currentPage === 'settings' && 'Application configuration and preferences'}
              </p>
            </div>

            {/* Page Content */}
            <div className="animate-slide-in" style={{ animationDelay: '100ms' }}>
              {renderPage()}
            </div>
          </div>
        </main>
      </div>
    </QueryClientProvider>
  );
}

export default App;
