import React from 'react';
import { useQuery, useMutation } from '@apollo/client';

import { ToDos } from '../../types/ToDos';
import { UpdateToDo, UpdateToDoVariables } from '../../types/UpdateToDo';

import * as queries from '../../graphql/queries';
import * as mutations from '../../graphql/mutations';

import ToDoList from '../ToDoList';

import './App.css';

function App() {
  const { data } = useQuery<ToDos>(queries.AllToDos);
  const [updateToDo] = useMutation<UpdateToDo, UpdateToDoVariables>(
    mutations.UpdateToDo,
  );

  const { toDos } = data ?? {};

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
        {toDos && (
          <ToDoList toDos={toDos} handleToDoUpdate={handleToDoUpdate} />
        )}
      </header>
    </div>
  );
}

export default App;
