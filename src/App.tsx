import { useState, lazy, Suspense } from 'react';
import { AnimatePresence } from 'motion/react';
import './index.css';
import SmoothScrollProvider from './components/SmoothScroll';
import ScrollProgress from './components/ScrollProgress';
import Navbar from './components/Navbar';
import Preloader from './components/Preloader';

// Lazy load all heavy section components for smaller initial bundle
const Hero = lazy(() => import('./components/Hero'));
const About = lazy(() => import('./components/About'));
const Philosophy = lazy(() => import('./components/Philosophy'));
const Skills = lazy(() => import('./components/Skills'));
const FeaturedProjects = lazy(() => import('./components/FeaturedProjects'));
const Projects = lazy(() => import('./components/Projects'));
const Experience = lazy(() => import('./components/Experience'));
const Certifications = lazy(() => import('./components/Certifications'));
const Contact = lazy(() => import('./components/Contact'));
const Footer = lazy(() => import('./components/Footer'));

// Minimal fallback to prevent layout shift while lazy sections load
const SectionFallback = () => (
  <div className="min-h-[50vh] flex items-center justify-center">
    <div className="w-8 h-8 border-2 border-text-primary/20 border-t-accent rounded-full animate-spin" />
  </div>
);

function App() {
  const [loaded, setLoaded] = useState(false);

  return (
    <SmoothScrollProvider>
      <div className="relative min-h-screen bg-background text-text-primary selection:bg-accent/30">
        <AnimatePresence>
          {!loaded && <Preloader onComplete={() => setLoaded(true)} />}
        </AnimatePresence>
        {/* GPU-Optimized Gradient Background Mesh */}
        <div className="gradient-mesh">
          <div className="gradient-blob blob-1" />
          <div className="gradient-blob blob-2" />
          <div className="gradient-blob blob-3" />
        </div>
        <ScrollProgress />
        <Navbar />
        <main>
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Hero />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <About />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Philosophy />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Skills />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <FeaturedProjects />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Projects />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Experience />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Certifications />
            </Suspense>
          </div>
          <div className="section-divider" />
          <div className="section-wrap">
            <Suspense fallback={<SectionFallback />}>
              <Contact />
            </Suspense>
          </div>
        </main>
        <Suspense fallback={<SectionFallback />}>
          <Footer />
        </Suspense>
      </div>
    </SmoothScrollProvider>
  );
}

export default App;
