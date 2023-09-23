import { KumaRegistry } from '@kuma-ui/next-plugin/registry';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Discover VST',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element {
  return (
    <html lang='en'>
      <body>
        <KumaRegistry>{children}</KumaRegistry>
      </body>
    </html>
  );
}
