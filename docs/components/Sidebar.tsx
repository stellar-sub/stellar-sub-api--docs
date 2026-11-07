"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navigation = [
  {
    title: "Getting Started",
    links: [{ title: "Introduction", href: "/docs/getting-started" }],
  },
  {
    title: "Concepts",
    links: [
      { title: "Authorization Model", href: "/docs/concepts/authorization-model" },
      { title: "Security Model", href: "/docs/concepts/security-model" },
    ],
  },
  {
    title: "Contracts",
    links: [
      { title: "Subscription Contract", href: "/docs/contracts/subscription" },
      { title: "Plan Contract", href: "/docs/contracts/plan" },
      { title: "Registry Contract", href: "/docs/contracts/registry" },
    ],
  },
  {
    title: "API Reference",
    links: [
      { title: "Overview", href: "/docs/api/overview" },
      { title: "Plans", href: "/docs/api/plans" },
      { title: "Subscriptions", href: "/docs/api/subscriptions" },
      { title: "Stats", href: "/docs/api/stats" },
    ],
  },
  {
    title: "Guides",
    links: [
      { title: "Merchant Guide", href: "/docs/merchant-guide" },
      { title: "Subscriber Guide", href: "/docs/subscriber-guide" },
      { title: "Integration Guide", href: "/docs/integration" },
    ],
  },
];

export default function Sidebar() {
  const pathname = usePathname();

  return (
    <nav className="w-64 border-r border-gray-200 dark:border-gray-800 p-6 overflow-y-auto">
      <div className="mb-8">
        <Link href="/" className="text-2xl font-bold text-primary">
          Stellar Subscriptions
        </Link>
      </div>
      {navigation.map((section) => (
        <div key={section.title} className="mb-8">
          <h3 className="text-sm font-semibold text-gray-900 dark:text-gray-100 uppercase tracking-wider mb-3">
            {section.title}
          </h3>
          <ul className="space-y-2">
            {section.links.map((link) => {
              const isActive = pathname === link.href;
              return (
                <li key={link.href}>
                  <Link
                    href={link.href}
                    className={`block px-3 py-2 rounded-md text-sm ${
                      isActive
                        ? "bg-primary text-white"
                        : "text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800"
                    }`}
                  >
                    {link.title}
                  </Link>
                </li>
              );
            })}
          </ul>
        </div>
      ))}
    </nav>
  );
}
