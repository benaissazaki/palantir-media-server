import MediaList from '../../components/MediaList';
import style from './HomePage.module.css';

const HomePage = () => {
  return (
    <>
      <div className={style.container}>
        <MediaList />
      </div>
    </>
  );
};

export default HomePage;
