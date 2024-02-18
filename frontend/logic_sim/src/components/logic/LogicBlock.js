import React, {useState} from 'react';
import { Handle, useUpdateNodeInternals, Position } from 'reactflow';
import './LogicBlockStyle.css'
import BlockInput from './BlockInput';

const componentHeight_px = 179 // TODO: This is dumb af

function LogicBlock({data, isConnectable}) {
    const updateNodeInternals = useUpdateNodeInternals();
    const [handleCount, setHandleCount] = useState(0);

    return (
        <div className='base-logic-block'>
            <div style={{'justify-content': 'space-around', display: 'flex'}}>
            {Array.from({length: 3}).map((_, index) => (
                        <Handle
                        key={index}
                        type="target"
                        position="left"
                        id={`handle-${index}`}
                      />
            ))}
            </div>
            <div>AND</div>
            <Handle type="source" position={Position.Right} />
        </div>
    );
};

export default LogicBlock;