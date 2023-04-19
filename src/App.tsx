import { ReactLocation, Router } from '@tanstack/react-location';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { homeDir, resolve } from '@tauri-apps/api/path';
import { useEffect, useState } from 'react';
import { FaImages } from 'react-icons/fa';

const IMAGE_EXTENSIONS = ['png', 'jpg', 'jpeg'];
const DEFAULT_QUALITY = 50;
const imageDirName = 'imageshrink';
const resizeCommand = 'minimize_image';
const appName = 'ImageShrink';

const Home = () => {
  const [outputPath, setOutputPath] = useState<string>();
  const [imagePath, setImagePath] = useState<string>('');
  const [quality, setQuality] = useState<number>(DEFAULT_QUALITY);

  const fetchHomeDirPath = async () => {
    const homeDirPath = await homeDir();
    const path = await resolve(homeDirPath, imageDirName);
    setOutputPath(path);
  };

  useEffect(() => {
    fetchHomeDirPath();
  }, []);

  const uploadImage = async () => {
    try {
      const imagePath = await open({
        filters: [
          {
            name: 'Image',
            extensions: IMAGE_EXTENSIONS,
          },
        ],
      });

      setImagePath(imagePath as string);
    } catch (error) {
      console.error(error);
    }
  };

  const handleQualityChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setQuality(parseInt(event.target.value));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await invoke(resizeCommand, { imagePath, quality });
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className="container center-align">
      <h3>
        <FaImages /> {appName}
      </h3>
      <p>Choose an image to resize</p>
      <form onSubmit={handleSubmit}>
        <div className="file-field input-field">
          <div className="btn">
            <span onClick={uploadImage}>Browse</span>
          </div>

          <div className="file-path-wrapper">
            <input
              type="text"
              className="file-path validate"
              placeholder="Upload file"
              value={imagePath}
              onChange={e => setImagePath(e.target.value)}
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
