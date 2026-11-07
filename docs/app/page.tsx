import Link from "next/link";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center p-8">
      <div className="max-w-4xl text-center">
        <h1 className="text-6xl font-bold mb-6 bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
          Stellar Subscriptions
        </h1>
        <p className="text-xl text-gray-600 dark:text-gray-400 mb-8">
          Recurring payments on Stellar with bounded, revocable authorizations.
          <br />
          Secure, transparent, and fully on-chain.
        </p>
        <div className="flex gap-4 justify-center">
          <Link
            href="/docs/getting-started"
            className="bg-primary text-white px-6 py-3 rounded-lg font-semibold hover:bg-primary/90 transition"
          >
            Get Started
          </Link>
          <Link
            href="/docs/concepts/authorization-model"
            className="bg-gray-200 dark:bg-gray-800 text-gray-900 dark:text-gray-100 px-6 py-3 rounded-lg font-semibold hover:bg-gray-300 dark:hover:bg-gray-700 transition"
          >
            Learn the Model
          </Link>
        </div>
        <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8 text-left">
          <div className="border border-gray-200 dark:border-gray-700 rounded-lg p-6">
            <h3 className="text-xl font-bold mb-2">🔒 Secure by Design</h3>
            <p className="text-gray-600 dark:text-gray-400">
              Subscribers control exactly how much can be charged and when they can revoke access.
            </p>
          </div>
          <div className="border border-gray-200 dark:border-gray-700 rounded-lg p-6">
            <h3 className="text-xl font-bold mb-2">⚡ Simple Integration</h3>
            <p className="text-gray-600 dark:text-gray-400">
              RESTful API and comprehensive documentation make integration straightforward.
            </p>
          </div>
          <div className="border border-gray-200 dark:border-gray-700 rounded-lg p-6">
            <h3 className="text-xl font-bold mb-2">🌐 Fully On-Chain</h3>
            <p className="text-gray-600 dark:text-gray-400">
              All authorization logic enforced by smart contracts on Stellar.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
