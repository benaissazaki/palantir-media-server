import style from './EmptyList.module.css';

const EmptyList = () => (
  <div className={style.container}>
    <img src="/icons/media-not-found.svg" />
    <p>Found no media file in the media directories specified in settings</p>
  </div>
);

export default EmptyList;
