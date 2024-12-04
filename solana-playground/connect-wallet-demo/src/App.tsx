import { useMemo} from 'react'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { UnsafeBurnerWallet } from '@solana/wallet-adapter-wallets'
import { WalletModalProvider, WalletMutiButton, WalletDisconnectButton } from '@solaan/wallet-adapter-react-ui'

const SolanaProvider: FunctionComponent<{ children:ReactNode}> = ({ children }) => {
  const network = WalletAdapterNetwork.DevNet
  const wallets = useMemo(() => [
    new UnsafeBurnerWallet()
  ], [network])
  
  return (
    <ConnectionProvider>
    <WalletProvider wallets={wallets}>
      <WalletModalProvider>
        <WalletMultiButton />
        <WalletDisconnectButton />
        {children}
      </WalletModalProvider>
    </WalletProvider>
    </ConnectionProvider>
  )
  
}

function App() {
  return (
    <SolanaProvider>
      Hello world!
    </SolanaProvider>
  )
}

export default App
