import React from 'react';
import ReactFlow, { Background, Controls, useNodesState } from 'reactflow';
import 'reactflow/dist/style.css';
import LogicBlock from './logic/LogicBlock';
 
const nodeTypes = {
  logic: LogicBlock,
}

const snapGrid = [10, 10];

const testNodes = [
  {
    id: '1',
    type: 'logic',
    data: { value: 122},
    position: {x: 0, y: 0},
  },
]


function LogicSandbox() {
  const [nodes, _, onNodesChange] = useNodesState(testNodes);

  return (
    <ReactFlow 
     nodeTypes={nodeTypes}
     nodes={nodes}
     snapGrid={snapGrid}>
        <Background color="#000000" />
        <Controls />
    </ReactFlow>
  );
};

export default LogicSandbox;