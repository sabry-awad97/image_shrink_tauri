import { homeDir, resolve } from '@tauri-apps/api/path';
import { useEffect, useState } from 'react';

const imageDirName = 'imageshrink';

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
      <div className="card output">
        <div className="card-content">
          Output Path: <span id="output-path"> {outputPath} </span>
        </div>
      </div>
    </div>
  );
};

export default Home;
