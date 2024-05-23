import React, { useEffect, useCallback } from 'react';

import AlgorithmFooter from "./AlgorithmFooter";
import '../assets/styles/AlgorithmPanel.css';

const AlgorithmPanel = (props) => {
    const {cycles, currentStep, setCycles} = props;

    const fetchCycles = () => {
        const body = {
            sort: "BubbleSort",
            arr: [5, 4, 3, 2, 1],
        };

        fetch('http://localhost:8080/api/algorithm/sort', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(body),
        })
        .then(response => {
            if (response.ok) {
                return response.json();
            }
            throw new Error('Request failed');
        })
        .then(data => {
            setCycles([...cycles, ...data.cycles]);
        })
        .catch(error => console.error(error));
    }

    useEffect(() => {
        fetchCycles();
    }, []);

    return (
        <div className="algorithmPanel">
            <h1>Algorithm Panel</h1>
            <AlgorithmFooter />
        </div>
    )
}

export default AlgorithmPanel;