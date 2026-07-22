import { motion } from 'framer-motion';
import {
  Cpu, Globe, Database, Layers, Terminal,
  Box, GitBranch, Server, Code, Brain, Network, Package,
  Cloud, Wrench, RefreshCw, Sparkles, Languages
} from 'lucide-react';
import TiltCard from './TiltCard';
import SectionGlow from './SectionGlow';

const skills = [
  // Languages
  { name: 'Python', icon: Terminal, category: 'Language' },
  { name: 'JavaScript', icon: Globe, category: 'Language' },
  { name: 'Rust', icon: Code, category: 'Language' },
  { name: 'C++', icon: Code, category: 'Language' },
  // AI / ML
  { name: 'Machine Learning', icon: Cpu, category: 'AI / ML' },
  { name: 'Deep Learning', icon: Layers, category: 'AI / ML' },
  { name: 'Generative AI', icon: Sparkles, category: 'AI / ML' },
  { name: 'Natural Language Processing', icon: Languages, category: 'AI / ML' },
  { name: 'Large Language Models (LLMs)', icon: Brain, category: 'AI / ML' },
  // Automation
  { name: 'Automation', icon: Brain, category: 'Automation' },
  { name: 'n8n', icon: Network, category: 'Automation' },
  // System
  { name: 'Linux', icon: Server, category: 'System' },
  // DevOps
  { name: 'Git', icon: GitBranch, category: 'DevOps' },
  { name: 'Docker', icon: Package, category: 'DevOps' },
  { name: 'DevOps', icon: Wrench, category: 'DevOps' },
  { name: 'CI/CD', icon: RefreshCw, category: 'DevOps' },
  // Cloud
  { name: 'Cloud Computing', icon: Cloud, category: 'Cloud' },
  // CS
  { name: 'Data Structures', icon: Database, category: 'CS' },
  { name: 'Algorithms', icon: GitBranch, category: 'CS' },
  // Tool
  { name: 'OpenCode', icon: Box, category: 'Tool' },
];

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: { staggerChildren: 0.06 },
  },
};

const cardVariant = {
  hidden: { opacity: 0, y: 30, scale: 0.95 },
  show: { opacity: 1, y: 0, scale: 1, transition: { duration: 0.5 } },
};

export default function Skills() {
  return (
    <section id="skills" className="pt-20 pb-24 md:pt-32 md:pb-32 px-6 relative overflow-hidden">
      <SectionGlow color="#00D4AA" position="bottom-left" size="lg" opacity={0.3} />
      <div className="max-w-6xl mx-auto relative z-10">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-16"
        >
          <h2 className="text-sm font-semibold text-accent-teal uppercase tracking-[0.2em] mb-4 font-mono-accent">
            Expertise
          </h2>
          <h3 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">
            Skills & Technologies
          </h3>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            A powerful toolkit spanning <span className="font-mono-accent text-accent-teal">AI/ML</span>, 
            automation, and software engineering — honed through academic excellence and real-world execution.
          </p>
        </motion.div>

        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={{ once: true, margin: '-50px' }}
          className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4"
        >
          {skills.map((skill) => (
            <motion.div key={skill.name} variants={cardVariant}>
              <TiltCard className="h-full">
                <div className="group h-full p-6 rounded-2xl glass-card breathe-border cursor-pointer">
                  <skill.icon
                    size={28}
                    className="text-text-secondary group-hover:text-accent-teal transition-colors duration-300 mb-4"
                  />
                  <h4 className="font-semibold text-text-primary mb-2 text-sm md:text-base">{skill.name}</h4>
                  <span className="text-xs text-text-secondary font-mono-accent">{skill.category}</span>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </motion.div>
      </div>
    </section>
  );
}
