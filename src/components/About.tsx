import { motion, type Variants } from 'framer-motion';
import SectionGlow from './SectionGlow';
import { easeSmooth, staggerContainer, viewportOnce } from '../lib/motion';

const imageVariants: Variants = {
  hidden: { scale: 0.9, opacity: 0, rotateY: 5 },
  show: {
    scale: 1,
    opacity: 1,
    rotateY: 0,
    transition: { duration: 1.5, ease: easeSmooth },
  },
};

const textVariants: Variants = {
  hidden: { y: 50, opacity: 0 },
  show: {
    y: 0,
    opacity: 1,
    transition: { duration: 1, ease: easeSmooth },
  },
};

const staggerContainerVariants = staggerContainer(0.1);

export default function About() {
  return (
    <section id="about" className="pt-32 md:pt-48 pb-12 md:pb-20 px-6 relative bg-background border-b border-border overflow-hidden">
      <SectionGlow color="#FF9933" position="top-right" size="lg" opacity={0.3} />
      <div className="max-w-7xl mx-auto relative z-10">
        <div className="grid lg:grid-cols-2 gap-16 lg:gap-24 items-start">
          {/* Left: Image */}
          <motion.div
            className="about-image-container relative"
            variants={imageVariants}
            initial="hidden"
            whileInView="show"
            viewport={viewportOnce}
          >
            <div className="aspect-[4/5] relative overflow-hidden bg-surface border border-text-primary/10 p-2">
              <img
                src="/avatar.jfif"
                alt="Avneesh Jadhav"
                className="w-full h-full object-cover filter grayscale hover:grayscale-0 transition-all duration-700"
                loading="lazy"
              />
            </div>
            {/* Minimal Accents */}
            <div className="absolute -bottom-4 -right-4 w-24 h-24 bg-gradient-to-br from-accent to-accent-glow blur-2xl opacity-50 pointer-events-none" />
          </motion.div>

          {/* Right: Content */}
          <motion.div
            className="flex flex-col justify-center h-full"
            variants={staggerContainerVariants}
            initial="hidden"
            whileInView="show"
            viewport={viewportOnce}
          >
            <motion.h2
              variants={textVariants}
              className="about-text-reveal text-5xl md:text-7xl font-bold tracking-tighter text-text-primary mb-8 uppercase leading-[0.9]"
            >
              Hi, I'm <span className="text-gradient">Avneesh.</span>
            </motion.h2>

            <motion.p
              variants={textVariants}
              className="about-text-reveal text-lg text-text-secondary leading-relaxed mb-4"
            >
              Technology has always fascinated me from the start. From opening up toys as a child to dissecting complex projects these days, it's always interesting to see how things work.
            </motion.p>

            <motion.p
              variants={textVariants}
              className="about-text-reveal text-lg text-text-secondary leading-relaxed mb-4"
            >
              In college, theory was good, but there came a point where I shifted from studying text to actually engineering solutions.
            </motion.p>

            <motion.p
              variants={textVariants}
              className="about-text-reveal text-lg text-text-secondary leading-relaxed mb-12"
            >
              Which brings me to the present. Currently, I'm focused on AI implementation.
            </motion.p>

            <motion.p
              variants={textVariants}
              className="about-text-reveal text-lg text-text-secondary leading-relaxed border-t border-border pt-8"
            >
              I am based out of <strong className="text-text-primary font-medium">Pune, Maharashtra, India</strong>.
            </motion.p>
          </motion.div>
        </div>
      </div>
    </section>
  );
}