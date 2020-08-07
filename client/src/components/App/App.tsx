import React, { useRef } from 'react';
import { useQuery, useMutation } from '@apollo/client';
import { TextField } from '@material-ui/core';

import { allToDos as allToDosQuery, AllToDos } from '../../graphql/queries';
import {
  updateToDo as updateToDoMutation,
  UpdateToDo,
  UpdateToDoVariables,
  addToDo as addToDoMutation,
  AddToDo,
  AddToDoVariables,
} from '../../graphql/mutations';

import AddButton from '../AddButton';
import List from '../List';
import ToDoItem from '../ToDoItem';

import './App.css';

function App() {
  const textField = useRef({ value: '' });
  const { data: { toDos = [] } = {} } = useQuery<AllToDos>(allToDosQuery);
  const [updateToDo] = useMutation<UpdateToDo, UpdateToDoVariables>(
    updateToDoMutation,
  );
  const [addToDo] = useMutation<AddToDo, AddToDoVariables>(addToDoMutation);

  const handleUpdateToDo = ({ id, done, label }: UpdateToDoVariables) => {
    updateToDo({
      variables: {
        id,
        done,
        label,
      },
    });
  };

  const handleAddToDo = (label: string) => {
    addToDo({
      variables: {
        label,
      },
    });
  };

  return (
    <div className="App">
      <header className="App-header">
        <List
          items={toDos}
          renderItem={(item) => (
            <ToDoItem
              key={item.id}
              toDo={item}
              handleDoneChange={(id, done) => handleUpdateToDo({ id, done })}
              handleLabelChange={(id, label) => handleUpdateToDo({ id, label })}
            />
          )}
        />
        <TextField inputRef={textField} />
        <AddButton onClick={() => handleAddToDo(textField.current.value)} />
      </header>
    </div>
  );
}

export default App;
