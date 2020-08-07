import React, { useRef } from 'react';
import { TextField } from '@material-ui/core';

import { UpdateToDoVariables } from '../../graphql/mutations';

import {
  useMutationAddToDo,
  useMutationUpdateToDo,
  useQueryAllToDos,
} from '../../hooks';

import AddButton from '../AddButton';
import List from '../List';
import ToDoItem from '../ToDoItem';

import './App.css';

function App() {
  const textField = useRef({ value: '' });
  const { data: { toDos = [] } = {} } = useQueryAllToDos();
  const [updateToDo] = useMutationUpdateToDo();
  const [addToDo] = useMutationAddToDo();

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
              handleDoneChange={(done) =>
                handleUpdateToDo({ id: item.id, done })
              }
              handleLabelChange={(label) =>
                handleUpdateToDo({ id: item.id, label })
              }
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
