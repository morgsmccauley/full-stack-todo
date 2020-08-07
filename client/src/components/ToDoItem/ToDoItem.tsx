import React, { useRef } from 'react';
import { Checkbox, TextField } from '@material-ui/core';

import { ToDos_toDos as ToDos } from '../../types/ToDos';

type ToDoItemProps = {
  toDo: ToDos;
  handleDoneChange: (done: boolean) => void;
  handleLabelChange: (label: string) => void;
};

const ToDoItem = ({
  toDo,
  handleDoneChange,
  handleLabelChange,
}: ToDoItemProps) => {
  const textFieldRef = useRef({ value: toDo.label });
  return (
    <div key={toDo.id} className="to-do">
      <Checkbox
        checked={toDo.done}
        onChange={(e) => handleDoneChange(e.target.checked)}
      />
      <TextField
        inputRef={textFieldRef}
        defaultValue={toDo.label}
        onBlur={() => handleLabelChange(textFieldRef.current.value)}
      />
    </div>
  );
};

export default ToDoItem;
