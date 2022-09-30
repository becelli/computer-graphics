import type { AppProps } from 'next/app';

import '../styles/globals.css';
// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
	return (
		<div className='bg-white dark:bg-black text-black dark:text-white h-screen w-screen'>
			<Component {...pageProps} />
		</div>
	);
}
