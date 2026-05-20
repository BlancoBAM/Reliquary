// 'use client'
import React from "react"
import { Providers } from "../src/components/ThemeContext"
import '../styles/globals.css'
import {Metadata} from 'next'

export const metadata:Metadata = {
  title: 'Reliquary',
  description: 'Reliquary — the default file manager for Lilith Linux. Fast, dark, and intelligent.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html suppressHydrationWarning lang="en" className="dark">
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover, interactive-widget=resizes-content"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous"/>
      </head>
      <body className="bg-background text-foreground">
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  )
}
