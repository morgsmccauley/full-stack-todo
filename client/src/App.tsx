import React from 'react';
import { useQuery } from '@apollo/client';
import gql from 'graphql-tag';

import './App.css';

const TO_DOS = gql`
  query {
    toDos {
      id
      label
      done
    }
  }
`;

type ToDo = {
  id: String,
  label: String,
  done: Boolean
}

function App() {
  const { data } = useQuery(TO_DOS);
  return (
    <div className="App">
      <header className="App-header">
        {data?.toDos.map((toDo: ToDo) => (
          <>
            <p>{toDo.id}</p>
            <p>{toDo.label}</p>
            <p>{toDo.done.toString()}</p>
          </>
        ))}
      </header>
    </div>
  );
}

export default App;
