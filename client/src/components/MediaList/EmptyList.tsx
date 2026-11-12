import { FolderIcon } from '../Icons';
import style from './EmptyList.module.css';

const EmptyList = () => (
  <div className={style.container}>
    <FolderIcon width={36} height={36} className={style.icon} />
    <h1 className={style.title}>No media found</h1>
    <p className={style.text}>
      Add directories containing videos or music from the Palantir Media Server app, then refresh this page.
    </p>
  </div>
);

export default EmptyList;
