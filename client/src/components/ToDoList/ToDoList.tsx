import React from 'react';

import { ToDos_toDos as ToDos } from '../../types/ToDos';
import ToDoItem from '../ToDoItem';

type HandleToDoUpdate = ({
  id,
  done: label,
}: {
  id: string;
  label?: string;
  done?: boolean;
}) => void;

type ToDoListProps = {
  toDos: ToDos[];
  handleToDoUpdate: HandleToDoUpdate;
};

const ToDoList = ({ toDos, handleToDoUpdate }: ToDoListProps) => (
  <>
    {toDos.map((toDo) => (
      <ToDoItem
        toDo={toDo}
        handleDoneChange={(id, done) => handleToDoUpdate({ id, done })}
        handleLabelChange={(id, label) => handleToDoUpdate({ id, label })}
      />
    ))}
  </>
);

export default ToDoList;
