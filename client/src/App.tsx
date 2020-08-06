import React from 'react';
import { useQuery, useMutation } from '@apollo/client';
import gql from 'graphql-tag';

import { ToDos, ToDos_toDos } from './types/ToDos';
import { UpdateToDo, UpdateToDoVariables } from './types/UpdateToDo';

import './App.css';

const TO_DOS = gql`
  query ToDos {
    toDos {
      id
      label
      done
    }
  }
`;

const UPDATE_TO_DO = gql`
  mutation UpdateToDo($id: String!, $label: String, $done: Boolean) {
    updateToDo (id: $id, label: $label, done: $done) {
      id
      label
      done
    }
  }
`;

type ToDoProps = {
  toDo: ToDos_toDos,
  handleDoneChange: (id: string, done: boolean) => void,
  handleLabelChange: (id: string, label: string) => void,
}

type HandleToDoUpdate = ({ id, done: label }: { id: string, label?: string, done?: boolean }) => void;

const ToDo = ({ toDo, handleDoneChange, handleLabelChange }: ToDoProps) => {
  return (
    <div key={toDo.id} className="to-do">
      <input type="text" value={toDo.label} onChange={(e) => handleLabelChange(toDo.id, e.target.value)} />
      <input type="checkbox" checked={toDo.done} onChange={(e) => handleDoneChange(toDo.id, e.target.checked)} />
    </div>
  );
};

function App() {
  const { data } = useQuery<ToDos>(TO_DOS);

  const [updateToDo] = useMutation<UpdateToDo, UpdateToDoVariables>(UPDATE_TO_DO);

  const handleToDoUpdate: HandleToDoUpdate = ({ id, done, label }) => {
    updateToDo({
      variables: {
        id,
        done,
        label,
      },
    });
  }

  return (
    <div className="App">
      <header className="App-header">
        {data?.toDos.map((toDo) => (
          <ToDo
            toDo={toDo}
            handleDoneChange={(id, done) => handleToDoUpdate({ id, done })}
            handleLabelChange={(id, label) => handleToDoUpdate({ id, label })}
          />
        ))}
      </header>
    </div>
  );
}

export default App;
