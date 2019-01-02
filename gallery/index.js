import './style';
import { Component } from 'preact';
import Masonry, { ResponsiveMasonry } from 'react-responsive-masonry';

const styles = {
  error: {
    color: '#C11',
    fontSize: 20,
  },
};

const Image = ({ url, maxWidth }) => (
  <a href={url} target="__blank" rel="noopener noreferrer">
    <img src={url} style={{ padding: 15, maxWidth: 390 }} className="image" />
  </a>
);

const Gallery = ({ urls, viewportWidth }) => (
  <ResponsiveMasonry columnsCountBreakPoints={{ 350: 1, 750: 2, 1200: 3 }}>
    <Masonry>
      {urls.map((url, i) => (
        <Image url={url} key={i} maxWidth={viewportWidth / 3} />
      ))}
    </Masonry>
  </ResponsiveMasonry>
);

const getViewportWidth = () =>
  Math.max(document.documentElement.clientWidth, window.innerWidth || 0);

export default class App extends Component {
  constructor() {
    super();
    this.state = {};

    const urlParams = new URLSearchParams(window.location.search);
    const backgroundColor = urlParams.get('bgColor') || '#121219';
    document
      .getElementsByTagName('body')[0]
      .setAttribute('style', `background-color: ${backgroundColor};`);

    const encodedUrls = urlParams.get('urls');

    try {
      const urls = JSON.parse(atob(encodedUrls));

      this.state = { urls, backgroundColor, width: getViewportWidth() };
    } catch (e) {
      this.state.error = 'Unable to parse URLs string';
    }
  }

  render() {
    if (this.state.error) {
      return <span style={styles.error}>{this.state.error}</span>;
    } else {
      return <Gallery urls={this.state.urls} viewportWidth={this.state.width} />;
    }
  }
}
