
import React, { useState, useMemo } from 'react';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line, Bar } from 'react-chartjs-2';
import { RustGear } from './RustGear';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
);

// Generate 365 days of mock snapshot data
const generateMockData = () => {
  const data = [];
  let stars = 10000;
  let forks = 2000;
  let issues = 500;

  const today = new Date();

  for (let i = 365; i >= 0; i--) {
    const date = new Date(today);
    date.setDate(date.getDate() - i);

    // Add some random growth
    stars += Math.floor(Math.random() * 50);
    forks += Math.floor(Math.random() * 10);
    issues += Math.floor(Math.random() * 5) - 2; // Issues can go down

    data.push({
      date: date.toISOString().split('T')[0].substring(5), // MM-DD
      monthKey: date.toISOString().substring(0, 7), // YYYY-MM
      stars,
      forks,
      issues,
    });
  }
  return data;
};

const MOCK_SNAPSHOTS = generateMockData();

type Dimension = 'stars' | 'forks' | 'issues';

export const RepoTrend: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'DELTAS' | 'SNAPSHOT'>('DELTAS');
  const [deltaTimeframe, setDeltaTimeframe] = useState<'weekly' | 'monthly'>('weekly');
  const [snapshotTimeframe, setSnapshotTimeframe] = useState<'monthly' | 'yearly'>('monthly');
  const [activeDimension, setActiveDimension] = useState<Dimension>('stars');

  const chartData = useMemo(() => {
    if (activeTab === 'SNAPSHOT') {
      if (snapshotTimeframe === 'yearly') {
        // Group by month and calculate average
        const monthlyGroups: Record<string, { stars: number, forks: number, issues: number, count: number }> = {};
        MOCK_SNAPSHOTS.forEach(d => {
          if (!monthlyGroups[d.monthKey]) {
            monthlyGroups[d.monthKey] = { stars: 0, forks: 0, issues: 0, count: 0 };
          }
          monthlyGroups[d.monthKey].stars += d.stars;
          monthlyGroups[d.monthKey].forks += d.forks;
          monthlyGroups[d.monthKey].issues += d.issues;
          monthlyGroups[d.monthKey].count += 1;
        });

        const sortedMonths = Object.keys(monthlyGroups).sort().slice(-12);

        return {
          labels: sortedMonths,
          datasets: [
            {
              label: 'Stars',
              data: sortedMonths.map(m => Math.round(monthlyGroups[m].stars / monthlyGroups[m].count)),
              borderColor: '#4ade80',
              backgroundColor: '#4ade80',
              tension: 0.1,
              hidden: activeDimension !== 'stars',
            },
            {
              label: 'Forks',
              data: sortedMonths.map(m => Math.round(monthlyGroups[m].forks / monthlyGroups[m].count)),
              borderColor: '#a78bfa',
              backgroundColor: '#a78bfa',
              tension: 0.1,
              hidden: activeDimension !== 'forks',
            },
            {
              label: 'Issues',
              data: sortedMonths.map(m => Math.round(monthlyGroups[m].issues / monthlyGroups[m].count)),
              borderColor: '#fbbf24',
              backgroundColor: '#fbbf24',
              tension: 0.1,
              hidden: activeDimension !== 'issues',
            }
          ]
        };
      } else {
        // Monthly snapshot
        const snapshots = MOCK_SNAPSHOTS.slice(-30);
        return {
          labels: snapshots.map(d => d.date),
          datasets: [
            {
              label: 'Stars',
              data: snapshots.map(d => d.stars),
              borderColor: '#4ade80',
              backgroundColor: '#4ade80',
              tension: 0.1,
              hidden: activeDimension !== 'stars',
            },
            {
              label: 'Forks',
              data: snapshots.map(d => d.forks),
              borderColor: '#a78bfa',
              backgroundColor: '#a78bfa',
              tension: 0.1,
              hidden: activeDimension !== 'forks',
            },
            {
              label: 'Issues',
              data: snapshots.map(d => d.issues),
              borderColor: '#fbbf24',
              backgroundColor: '#fbbf24',
              tension: 0.1,
              hidden: activeDimension !== 'issues',
            }
          ]
        };
      }
    } else {
      // Deltas
      const days = deltaTimeframe === 'weekly' ? 7 : 30;
      const snapshots = MOCK_SNAPSHOTS.slice(-days - 1); // Need one extra for first delta

      const labels = [];
      const starsDelta = [];
      const forksDelta = [];
      const issuesDelta = [];

      for (let i = 1; i < snapshots.length; i++) {
        labels.push(snapshots[i].date);
        starsDelta.push(snapshots[i].stars - snapshots[i - 1].stars);
        forksDelta.push(snapshots[i].forks - snapshots[i - 1].forks);
        issuesDelta.push(snapshots[i].issues - snapshots[i - 1].issues);
      }

      return {
        labels,
        datasets: [
          {
            label: 'stars_delta',
            data: starsDelta,
            backgroundColor: '#5eead4', // Teal-ish
            hidden: activeDimension !== 'stars',
          },
          {
            label: 'forks_delta',
            data: forksDelta,
            backgroundColor: '#a78bfa', // Purple
            hidden: activeDimension !== 'forks',
          },
          {
            label: 'open_issues_delta',
            data: issuesDelta,
            backgroundColor: '#fcd34d', // Orange/Yellow
            hidden: activeDimension !== 'issues',
          }
        ]
      };
    }
  }, [activeTab, deltaTimeframe, snapshotTimeframe, activeDimension]);

  const options = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'bottom' as const,
        onClick: (e: any, legendItem: any, legend: any) => {
          const index = legendItem.datasetIndex;
          const ci = legend.chart;

          // Set the clicked dataset to visible, others to hidden
          ci.data.datasets.forEach((d: any, i: number) => {
            ci.setDatasetVisibility(i, i === index);
          });
          ci.update();
        },
        labels: {
          usePointStyle: false,
          boxWidth: 40,
          boxHeight: 12,
          color: '#6b7280',
          font: {
            family: 'ui-sans-serif, system-ui, sans-serif',
            size: 12
          }
        }
      },
    },
    scales: {
      y: {
        grid: {
          color: '#f3f4f6',
        },
        border: {
          display: false,
        },
        ticks: {
          color: '#6b7280',
          font: {
            family: 'ui-sans-serif, system-ui, sans-serif',
            size: 12
          }
        }
      },
      x: {
        grid: {
          display: false,
        },
        border: {
          color: '#e5e7eb',
        },
        ticks: {
          color: '#6b7280',
          font: {
            family: 'ui-sans-serif, system-ui, sans-serif',
            size: 12
          }
        }
      }
    }
  };

  // Calculate per summary statistics
  const today = MOCK_SNAPSHOTS[MOCK_SNAPSHOTS.length - 1];
  const yesterday = MOCK_SNAPSHOTS[MOCK_SNAPSHOTS.length - 2];
  const lastWeek = MOCK_SNAPSHOTS[MOCK_SNAPSHOTS.length - 8];
  const lastMonth = MOCK_SNAPSHOTS[MOCK_SNAPSHOTS.length - 31];

  const dailyDelta = today[activeDimension] - yesterday[activeDimension];
  const weeklyDelta = Math.round((today[activeDimension] - lastWeek[activeDimension]) / 7);
  const monthlyDelta = Math.round((today[activeDimension] - lastMonth[activeDimension]) / 30);

  const formatDelta = (val: number) => {
    if (val > 0) return `+${val}`;
    if (val < 0) return `${val}`;
    return '0';
  };

  return (
    <div className="w-full flex flex-col mb-16">
      {/* Section Title */}
      <div className="mb-8 flex flex-col md:flex-row md:items-end justify-between gap-4">
        <h2 className="text-5xl md:text-7xl font-black font-sans text-ink dark:text-white leading-[0.8] tracking-tighter uppercase">
          Trend<br />
          <span className="text-transparent [-webkit-text-stroke:2px_#e5e7eb] dark:[-webkit-text-stroke:2px_rgba(255,255,255,0.2)]">Analysis</span>
        </h2>

        {/* Dimension Selector */}
        <div className="flex gap-2">
          {(['stars', 'forks', 'issues'] as Dimension[]).map(dim => (
            <button
              key={dim}
              onClick={() => setActiveDimension(dim)}
              className={`px-4 py-2 text-sm font-bold uppercase tracking-wider border-2 border-ink dark:border-dark-ink transition-colors ${activeDimension === dim
                  ? 'bg-ink text-white dark:bg-dark-ink dark:text-ink shadow-[4px_4px_0_0_#ea580c]'
                  : 'bg-white text-ink dark:bg-dark-paper dark:text-dark-ink hover:bg-gray-100 dark:hover:bg-gray-900'
                }`}
            >
              {dim}
            </button>
          ))}
        </div>
      </div>

      {/* Per Summary Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
        {[
          { label: 'Daily', value: dailyDelta },
          { label: 'Weekly (Avg/Day)', value: weeklyDelta },
          { label: 'Monthly (Avg/Day)', value: monthlyDelta }
        ].map((stat, idx) => (
          <div key={idx} className="relative bg-white dark:bg-dark-paper border-2 border-ink dark:border-dark-ink p-6 flex flex-col items-center justify-center overflow-hidden min-h-[160px] shadow-[4px_4px_0_0_#141414] dark:shadow-[4px_4px_0_0_#e4e3e0]">
            <RustGear className="absolute w-48 h-48 text-gray-100 dark:text-gray-800/50 opacity-50 pointer-events-none" />
            <div className="relative z-10 flex flex-col items-center">
              <span className="text-xs font-mono text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-2 bg-white/80 dark:bg-dark-paper/80 px-2">
                {stat.label}
              </span>
              <span className={`text-5xl font-black font-mono ${stat.value > 0 ? 'text-emerald-500' : stat.value < 0 ? 'text-rust-primary' : 'text-ink dark:text-dark-ink'}`}>
                {formatDelta(stat.value)}
              </span>
            </div>
          </div>
        ))}
      </div>

      {/* Tabs */}
      <div className="flex gap-6 mb-4 px-2">
        <button
          onClick={() => setActiveTab('DELTAS')}
          className={`text-sm font-bold tracking-wider pb-2 border-b-4 transition-colors ${activeTab === 'DELTAS' ? 'border-ink text-ink dark:border-dark-ink dark:text-dark-ink' : 'border-transparent text-gray-500 hover:text-ink dark:hover:text-dark-ink'}`}
        >
          DELTAS
        </button>
        <button
          onClick={() => setActiveTab('SNAPSHOT')}
          className={`text-sm font-bold tracking-wider pb-2 border-b-4 transition-colors ${activeTab === 'SNAPSHOT' ? 'border-ink text-ink dark:border-dark-ink dark:text-dark-ink' : 'border-transparent text-gray-500 hover:text-ink dark:hover:text-dark-ink'}`}
        >
          SNAPSHOT
        </button>
      </div>

      {/* Content Box - Recessed UI */}
      <div className="bg-gray-100 dark:bg-black border-2 border-ink dark:border-dark-ink shadow-[inset_4px_4px_0_0_#141414] dark:shadow-[inset_4px_4px_0_0_#e4e3e0] p-6">
        <div className="flex justify-end mb-6">
          {/* Timeframe selector for Deltas */}
          {activeTab === 'DELTAS' && (
            <div className="flex gap-2">
              <button
                onClick={() => setDeltaTimeframe('weekly')}
                className={`px-3 py-1 text-xs font-mono border-2 border-ink dark:border-dark-ink ${deltaTimeframe === 'weekly' ? 'bg-ink text-white dark:bg-dark-ink dark:text-ink' : 'bg-white text-ink dark:bg-dark-paper dark:text-dark-ink hover:bg-gray-100 dark:hover:bg-gray-900'}`}
              >
                Weekly
              </button>
              <button
                onClick={() => setDeltaTimeframe('monthly')}
                className={`px-3 py-1 text-xs font-mono border-2 border-ink dark:border-dark-ink ${deltaTimeframe === 'monthly' ? 'bg-ink text-white dark:bg-dark-ink dark:text-ink' : 'bg-white text-ink dark:bg-dark-paper dark:text-dark-ink hover:bg-gray-100 dark:hover:bg-gray-900'}`}
              >
                Monthly
              </button>
            </div>
          )}

          {/* Timeframe selector for Snapshot */}
          {activeTab === 'SNAPSHOT' && (
            <div className="flex gap-2">
              <button
                onClick={() => setSnapshotTimeframe('monthly')}
                className={`px-3 py-1 text-xs font-mono border-2 border-ink dark:border-dark-ink ${snapshotTimeframe === 'monthly' ? 'bg-ink text-white dark:bg-dark-ink dark:text-ink' : 'bg-white text-ink dark:bg-dark-paper dark:text-dark-ink hover:bg-gray-100 dark:hover:bg-gray-900'}`}
              >
                Monthly
              </button>
              <button
                onClick={() => setSnapshotTimeframe('yearly')}
                className={`px-3 py-1 text-xs font-mono border-2 border-ink dark:border-dark-ink ${snapshotTimeframe === 'yearly' ? 'bg-ink text-white dark:bg-dark-ink dark:text-ink' : 'bg-white text-ink dark:bg-dark-paper dark:text-dark-ink hover:bg-gray-100 dark:hover:bg-gray-900'}`}
              >
                Yearly
              </button>
            </div>
          )}
        </div>

        <div className="h-[400px] w-full">
          {activeTab === 'DELTAS' ? (
            <Bar data={chartData} options={options} />
          ) : (
            <Line data={chartData} options={options} />
          )}
        </div>
      </div>
    </div>
  );
};
