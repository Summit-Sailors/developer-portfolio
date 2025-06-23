"use client"

import Link from "next/link"
import { PERSONAL_INFO, NAVIGATION_LINKS } from "@/constants/data"
import { useState } from "react"
import { FiMenu, FiX } from "react-icons/fi"

export default function Header() {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false)

  return (
    <header className="fixed top-0 w-full z-50 backdrop-blur-md bg-black/30 border-b border-zinc-800">
      <div className="container mx-auto px-4 py-4 flex justify-between items-center">
        <Link
          href="/"
          className="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-cyan-500"
        >
          {PERSONAL_INFO.name.split(" ")[0]}.dev
        </Link>
        <nav className="hidden md:flex space-x-6">
          {NAVIGATION_LINKS.map((item) => (
            <Link key={item.name} href={item.href} className="text-zinc-400 hover:text-white transition-colors">
              {item.name}
            </Link>
          ))}
        </nav>
        <div className="md:hidden">
          <button
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            className="text-zinc-400 hover:text-white p-2"
            aria-label="Toggle menu"
          >
            {isMobileMenuOpen ? <FiX size={24} /> : <FiMenu size={24} />}
          </button>
        </div>
      </div>
      {/* Mobile Menu */}
      {isMobileMenuOpen && (
        <div className="md:hidden absolute top-full left-0 w-full bg-black/80 backdrop-blur-md border-b border-zinc-800">
          <nav className="flex flex-col items-center space-y-4 py-4">
            {NAVIGATION_LINKS.map((item) => (
              <Link
                key={item.name}
                href={item.href}
                onClick={() => setIsMobileMenuOpen(false)}
                className="text-zinc-300 hover:text-white text-lg transition-colors"
              >
                {item.name}
              </Link>
            ))}
          </nav>
        </div>
      )}
    </header>
  )
}
