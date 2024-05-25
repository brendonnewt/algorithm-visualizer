import React from 'react';
import StepPanel from './StepPanel';
import WorkPanel from './WorkPanel';
import '../assets/styles/AlgorithmSection.css';

const AlgorithmSection = ({cycles, currentCycle, currentStep, nextStep, prevStep, stepString}) => {
    return (
        <div className="algorithmSection">
            <StepPanel currentCycle={currentCycle} stepString={stepString}/>
            <WorkPanel 
            cycles={cycles}
            currentStep={currentStep}
            nextStep={nextStep}
            prevStep={prevStep}
            />
        </div>
    );
}

export default AlgorithmSection;