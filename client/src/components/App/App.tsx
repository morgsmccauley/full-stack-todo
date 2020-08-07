import React from 'react';
import { useQuery, useMutation } from '@apollo/client';

import { ToDos } from '../../types/ToDos';
import { UpdateToDo, UpdateToDoVariables } from '../../types/UpdateToDo';

import { AllToDos } from '../../graphql/queries';
import { UpdateToDo as UpdateToDoMutation } from '../../graphql/mutations';

import AddButton from '../AddButton';
import List from '../List';
import ToDoItem from '../ToDoItem';

import './App.css';

function App() {
  const { data: { toDos = [] } = {} } = useQuery<ToDos>(AllToDos);
  const [updateToDo] = useMutation<UpdateToDo, UpdateToDoVariables>(
    UpdateToDoMutation,
  );

  const handleToDoUpdate = ({ id, done, label }: UpdateToDoVariables) => {
    updateToDo({
      variables: {
        id,
        done,
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
              toDo={item}
              handleDoneChange={(id, done) => handleToDoUpdate({ id, done })}
              handleLabelChange={(id, label) => handleToDoUpdate({ id, label })}
            />
          )}
        />
        <AddButton />
      </header>
    </div>
  );
}

export default App;
