import { motion } from 'framer-motion';
import { Award, GraduationCap, Cpu, Cloud, Database, BookOpen, Sparkles, Terminal } from 'lucide-react';
import TiltCard from './TiltCard';

const certifications = [
  {
    title: 'Machine Learning Specialization',
    org: 'Stanford University',
    skills: ['Supervised ML', 'Advanced Learning', 'Unsupervised Learning', 'Recommenders'],
    icon: GraduationCap,
    color: 'from-red-500/20 to-orange-500/20',
    borderColor: 'border-red-500/30',
    iconColor: 'text-red-400',
  },
  {
    title: 'Fundamentals of Deep Learning',
    org: 'Nvidia',
    skills: ['Neural Networks', 'Deep Learning', 'GPU Acceleration'],
    icon: Cpu,
    color: 'from-green-500/20 to-emerald-500/20',
    borderColor: 'border-green-500/30',
    iconColor: 'text-green-400',
  },
  {
    title: 'OCI Generative AI Professional',
    org: 'Oracle',
    skills: ['GenAI', 'Cloud AI', 'LLM Deployment'],
    icon: Cloud,
    color: 'from-red-500/20 to-red-400/20',
    borderColor: 'border-red-500/30',
    iconColor: 'text-red-400',
  },
  {
    title: 'Oracle AI Vector Search Professional',
    org: 'Oracle',
    skills: ['Vector DB', 'RAG', 'Semantic Search'],
    icon: Database,
    color: 'from-red-500/20 to-red-400/20',
    borderColor: 'border-red-500/30',
    iconColor: 'text-red-400',
  },
  {
    title: 'AMCAT Certified Software Engineer',
    org: 'IT Services',
    skills: ['Software Engineering', 'Problem Solving', 'Code Quality'],
    icon: Award,
    color: 'from-blue-500/20 to-cyan-500/20',
    borderColor: 'border-blue-500/30',
    iconColor: 'text-blue-400',
  },
  {
    title: 'Model Context Protocol',
    org: 'Anthropic',
    skills: ['MCP', 'AI Integration', 'Tool Use'],
    icon: Sparkles,
    color: 'from-yellow-500/20 to-amber-500/20',
    borderColor: 'border-yellow-500/30',
    iconColor: 'text-yellow-400',
  },
  {
    title: 'MCP Advanced Topics',
    org: 'Anthropic',
    skills: ['Advanced MCP', 'AI Architecture', 'System Design'],
    icon: BookOpen,
    color: 'from-yellow-500/20 to-amber-500/20',
    borderColor: 'border-yellow-500/30',
    iconColor: 'text-yellow-400',
  },
  {
    title: 'Advanced Rust: Managing Projects',
    org: 'LinkedIn',
    skills: ['Rust', 'Project Management'],
    icon: Award,
    color: 'from-orange-500/20 to-amber-500/20',
    borderColor: 'border-orange-500/30',
    iconColor: 'text-orange-400',
  },
  {
    title: 'Advanced Linux: The Linux Kernel',
    org: 'LinkedIn',
    skills: ['Linux', 'System Administration'],
    icon: Terminal,
    color: 'from-emerald-500/20 to-teal-500/20',
    borderColor: 'border-emerald-500/30',
    iconColor: 'text-emerald-400',
  },
];

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: { staggerChildren: 0.08 },
  },
};

const cardVariant = {
  hidden: { opacity: 0, y: 30, scale: 0.95 },
  show: { opacity: 1, y: 0, scale: 1, transition: { duration: 0.5 } },
};

export default function Certifications() {
  return (
    <section id="certifications" className="pt-32 pb-24 md:pt-40 md:pb-32 px-6 relative">
      <div className="max-w-6xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-16"
        >
          <h2 className="text-sm font-semibold text-accent-teal uppercase tracking-[0.2em] mb-4 font-mono-accent">
            Credentials
          </h2>
          <h3 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">Certifications</h3>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            Industry-recognized certifications from <span className="font-mono-accent text-accent-teal">Stanford</span>,
            <span className="font-mono-accent text-accent-teal"> Nvidia</span>,
            <span className="font-mono-accent text-accent-teal"> Oracle</span>,
            <span className="font-mono-accent text-accent-teal"> LinkedIn</span>, and
            <span className="font-mono-accent text-accent-teal"> Anthropic</span>.
          </p>
        </motion.div>

        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={{ once: true, margin: '-50px' }}
          className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          {certifications.map((cert) => (
            <motion.div key={cert.title} variants={cardVariant}>
              <TiltCard className="h-full">
                <div className={`group h-full p-6 rounded-2xl bg-gradient-to-br ${cert.color} border ${cert.borderColor} backdrop-blur-sm transition-all duration-300 hover:border-opacity-80 cursor-pointer`}>
                  <div className="flex items-start justify-between mb-5">
                    <div className="p-3 rounded-xl bg-background/50 border border-white/10">
                      <cert.icon size={24} className={cert.iconColor} />
                    </div>
                    <span className="text-xs font-medium text-text-secondary bg-background/50 px-3 py-1 rounded-full border border-white/10 font-mono-accent">
                      {cert.org}
                    </span>
                  </div>

                  <h4 className="text-lg font-bold text-text-primary mb-4 group-hover:text-gradient transition-all duration-300">
                    {cert.title}
                  </h4>

                  <div className="flex flex-wrap gap-2">
                    {cert.skills.map((skill) => (
                      <span
                        key={skill}
                        className="px-2.5 py-1 text-xs font-medium rounded-full bg-background/60 border border-white/10 text-text-secondary font-mono-accent"
                      >
                        {skill}
                      </span>
                    ))}
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </motion.div>
      </div>
    </section>
  );
}
