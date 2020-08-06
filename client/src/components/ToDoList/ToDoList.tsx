import React, { Fragment } from 'react';

import { ToDos_toDos } from '../../types/ToDos';
import ToDoItem from '../ToDoItem';

type HandleToDoUpdate = ({ id, done: label }: { id: string, label?: string, done?: boolean }) => void;

type ToDoListProps = {
  toDos: ToDos_toDos[],
  handleToDoUpdate: HandleToDoUpdate,
};

const ToDoList = ({ toDos, handleToDoUpdate }: ToDoListProps) => (
  <Fragment>
    {toDos.map((toDo) => (
      <ToDoItem
        toDo={toDo}
        handleDoneChange={(id, done) => handleToDoUpdate({ id, done })}
        handleLabelChange={(id, label) => handleToDoUpdate({ id, label })}
      />
    ))}
  </Fragment>
);

export default ToDoList;
