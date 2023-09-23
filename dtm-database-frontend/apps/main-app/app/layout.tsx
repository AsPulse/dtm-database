import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: "Discover VST"
};


export default function RootLayout({ children }: { children: React.ReactNode }): JSX.Element {
  return (
    <html lang='en'>
      {children}
    </html>
  );
}
