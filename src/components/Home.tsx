import { homeDir, resolve } from '@tauri-apps/api/path';
import { useEffect, useState } from 'react';
import { appName, imageDirName } from '../constants';
import { FaImages } from 'react-icons/fa';

const Home = () => {
  const [outputPath, setOutputPath] = useState<string>('');

  const fetchHomeDirPath = async () => {
    const homeDirPath = await homeDir();
    const path = await resolve(homeDirPath, imageDirName);
    setOutputPath(path);
  };

  useEffect(() => {
    fetchHomeDirPath();
  }, []);

  return (
    <div className="container center-align">
      <h3>
        <FaImages /> {appName}
      </h3>
      <p>Choose an image to resize</p>
      <form>
        <div className="file-field input-field">
          <div className="btn">
            <span>Browse</span>
          </div>

          <div className="file-path-wrapper">
            <input
              type="text"
              className="file-path validate"
              placeholder="Upload file"
            />
          </div>
        </div>

        <p>
          <strong>Quality:</strong>
          <em>The lower the quality, the smaller the file size</em>
        </p>
        <p className="range-field">
          <input type="range" min="0" max="100" value="50" />
          <span id="quality-value">50</span>
        </p>

        <input type="submit" value="Resize" className="btn black" />
      </form>
      <div className="card output">
        <div className="card-content">
          Output Path: <span id="output-path"> {outputPath} </span>
        </div>
      </div>
    </div>
  );
};

export default Home;
