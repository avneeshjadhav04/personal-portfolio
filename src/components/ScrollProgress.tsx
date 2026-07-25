import { memo } from 'react';
import { motion, useScroll, useSpring } from 'motion/react';

const ScrollProgress = memo(function ScrollProgress() {
  const { scrollYProgress } = useScroll();
  const scaleX = useSpring(scrollYProgress, {
    stiffness: 100,
    damping: 30,
    restDelta: 0.001,
  });

  return (
    <motion.div
      className="fixed top-0 left-0 right-0 h-[4px] z-[60] origin-left"
      style={{
        scaleX,
        background: 'linear-gradient(90deg, #7B61FF, #A78BFA)',
      }}
    />
  );
});

export default ScrollProgress;
