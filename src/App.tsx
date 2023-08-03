import { ReactLocation, Router } from '@tanstack/react-location';
import './App.css';
import Home from './components/Home';
import About from './components/About';

const reactLocation = new ReactLocation();

const App = () => {
  return (
    <Router
      location={reactLocation}
      routes={[
        {
          path: '/',
          element: <Home />,
        },
        {
          path: '/about',
          element: <About />,
        },
      ]}
    />
  );
};

export default App;
