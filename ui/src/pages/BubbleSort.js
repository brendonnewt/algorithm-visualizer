import React from 'react';
import SortPage from '../components/SortPage';
import '../assets/styles/MainPanel.css';

const BubbleSort = () => {
    const stepString = "Compare elements at index i and i+1. If element at i is greater than element at i+1, swap them. Repeat this process for all elements in the array until no swaps are made.";
    const sort = "BubbleSort";

    return (
        <div className="mainPanel">
            <SortPage stepString={stepString} sort={sort}/>
        </div>
    );
}

export default BubbleSort;