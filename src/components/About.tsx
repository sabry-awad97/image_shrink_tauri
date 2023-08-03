import { FaImages } from 'react-icons/fa';
import { appName } from '../constants';

const About = () => {
  return (
    <div className="container about center-align">
      <h4>
        <FaImages /> {appName}
      </h4>
      <ul className="collection">
        <li className="collection-item">Developer: Dr.Sabry</li>
        <li className="collection-item">Version: 1.0.0</li>
      </ul>
    </div>
  );
};

export default About;
