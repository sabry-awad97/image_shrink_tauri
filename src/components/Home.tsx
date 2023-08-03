import { homeDir, resolve } from '@tauri-apps/api/path';
import { useEffect, useState } from 'react';
import { DEFAULT_QUALITY, appName, imageDirName } from '../constants';
import { FaImages } from 'react-icons/fa';

const Home = () => {
  const [outputPath, setOutputPath] = useState<string>('');
  const [quality, setQuality] = useState<number>(DEFAULT_QUALITY);

  const handleQualityChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setQuality(parseInt(event.target.value));
  };

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
          <input
            type="range"
            min="0"
            max="100"
            value={quality}
            onChange={handleQualityChange}
          />
          <span id="quality-value">{quality}</span>
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
