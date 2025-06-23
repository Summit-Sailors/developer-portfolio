import type React from "react"
import type { Metadata } from "next"
import { Inter } from "next/font/google"
import "./globals.css"
import Header from "@/components/layout/header"
import Footer from "@/components/layout/footer"
import { PERSONAL_INFO } from "@/constants/data"

const inter = Inter({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: `${PERSONAL_INFO.name} | Software Developer`,
  description: `Portfolio of ${PERSONAL_INFO.name}. ${PERSONAL_INFO.shortDescription}`,
    generator: 'v0.dev'
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" className="scroll-smooth">
      <body
        className={`${inter.className} bg-gradient-to-b from-zinc-900 to-black text-white min-h-screen flex flex-col`}
      >
        <Header />
        <main className="flex-grow pt-16">
          {" "}
          {/* pt-16 for fixed header */}
          {children}
        </main>
        <Footer />
      </body>
    </html>
  )
}
