import React from 'react';

import { ToDos_toDos } from '../../types/ToDos';

type ToDoItemProps = {
  toDo: ToDos_toDos,
  handleDoneChange: (id: string, done: boolean) => void,
  handleLabelChange: (id: string, label: string) => void,
}

const ToDoItem = ({ toDo, handleDoneChange, handleLabelChange }: ToDoItemProps) => {
  return (
    <div key={toDo.id} className="to-do">
      <input type="text" value={toDo.label} onChange={(e) => handleLabelChange(toDo.id, e.target.value)} />
      <input type="checkbox" checked={toDo.done} onChange={(e) => handleDoneChange(toDo.id, e.target.checked)} />
    </div>
  );
};

export default ToDoItem;
