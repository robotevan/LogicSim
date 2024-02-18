import React from 'react';
import {Handle, Position} from 'reactflow';

function BlockInput({inputNumber, inputTop}) {
    return (
        <Handle type="target" 
            position={Position.Left} 
            id={inputNumber}
        />
    )
}

export default BlockInput;