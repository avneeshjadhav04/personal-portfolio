import { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';

export default function Preloader({ onComplete }: { onComplete: () => void }) {
  const [phase, setPhase] = useState<'enter' | 'settle' | 'exit'>('enter');

  useEffect(() => {
    let done = false;

    const settleTimer = setTimeout(() => {
      if (!done) setPhase('settle');
    }, 800);

    const exitTimer = setTimeout(() => {
      if (!done) setPhase('exit');
    }, 1600);

    const doneTimer = setTimeout(() => {
      if (!done) {
        done = true;
        onComplete();
      }
    }, 2200);

    return () => {
      done = true;
      clearTimeout(settleTimer);
      clearTimeout(exitTimer);
      clearTimeout(doneTimer);
    };
  }, [onComplete]);

  const progressWidth = phase === 'settle' || phase === 'exit' ? '100%' : '0%';

  return (
    <AnimatePresence>
      {phase !== 'exit' && (
        <motion.div
          className="fixed inset-0 z-[9999] flex items-center justify-center bg-background"
          initial={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.5, ease: [0.4, 0, 0.2, 1] }}
        >
          <div className="relative flex flex-col items-center gap-8">
            {/* Name reveal */}
            <div className="flex items-center gap-1 overflow-hidden">
              {'Avneesh'.split('').map((char, i) => (
                <motion.span
                  key={i}
                  className="text-4xl md:text-6xl font-bold tracking-tight text-gradient"
                  initial={{ y: 80, opacity: 0, rotateX: -90 }}
                  animate={{ y: 0, opacity: 1, rotateX: 0 }}
                  transition={{
                    duration: 0.5,
                    delay: 0.1 + i * 0.05,
                    ease: [0.4, 0, 0.2, 1],
                  }}
                >
                  {char}
                </motion.span>
              ))}
            </div>

            {/* Progress bar */}
            <div className="w-48 h-[2px] bg-surface-light rounded-full overflow-hidden">
              <motion.div
                className="h-full bg-accent"
                initial={{ width: '0%' }}
                animate={{ width: progressWidth }}
                transition={{ duration: 0.8, ease: [0.4, 0, 0.2, 1] }}
              />
            </div>

            {/* Subtitle */}
            <motion.p
              className="text-sm text-text-secondary font-mono-accent tracking-widest uppercase"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.4, duration: 0.4 }}
            >
              Loading
            </motion.p>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
