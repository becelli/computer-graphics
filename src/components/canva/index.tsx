import React, { useRef, useEffect } from 'react';

// increment the counter
const Counter = ({ value: number }) => {
	const ref = useRef(null);

	useEffect(() => {
		ref.current.textContent = number;
	}, [number]);

	return <span ref={ref} />;
};

export default Counter;
