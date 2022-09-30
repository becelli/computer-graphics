import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Counter from '../components/canva';

function App() {
	const [counter, setCounter]: [number, Function] = useState(1);

	async function incrementCounter() {
		const result = await invoke('increment_counter', { counter });
		setCounter(result);
	}

	return (
		<div className='container mx-auto'>
			<div className='flex flex-col justify-center items-center h-screen'>
				<h1 className='text-6xl'>
					<Counter value={counter} />
				</h1>
				<button
					className='p-2 bg-red-600 border-md rounded-md absolute bottom-0 right-0 -translate-x-full'
					onClick={() => incrementCounter()}>
					Increment
				</button>
			</div>
		</div>
	);
}

export default App;
