import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Text,
  VStack,
  Grid,
  theme,
  Button,
  Table, Thead, Tbody, Tr, Th, Td
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';
import { Logo } from './Logo';

const WASMCLOUD_COLOR = "#00C389";
const backupData = [
  { id: 1, item: "Paper", quantity: 500 },
  { id: 2, item: "Printers", quantity: 10 },
  { id: 3, item: "Ink", quantity: 20 },
];

function App() {
  const [inventoryData, setInventoryData] = useState([]);

  const fetchData = () => {
    // Fetch data from your API endpoint
    fetch("your-api-url")
      .then((response) => response.json())
      .then((data) => setInventoryData(data))
      .catch((error) => {
        setInventoryData(backupData);
        console.error("Error fetching data:", error);
      });
  };

  useEffect(() => {
    fetchData(); // Fetch data when the component mounts
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid minH="100vh" p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <VStack spacing={4}>
            <Logo h="20vmin" pointerEvents="none" />
            <Text>
              Branch Dashboard
            </Text>
            <Box width="100%">
              <Button color={WASMCLOUD_COLOR} onClick={fetchData} mb={4}>Query Inventory</Button>
              <Box overflowX="auto" textAlign="center">
                <Table variant="simple" width="100%" maxWidth="50vw" mx="auto">
                  <Thead>
                    <Tr>
                      <Th>Item</Th>
                      <Th textAlign="right">Quantity</Th> {/* Right-align Quantity column */}
                    </Tr>
                  </Thead>
                  <Tbody>
                    {inventoryData.map((item) => (
                      <Tr key={item.id}>
                        <Td>{item.item}</Td>
                        <Td textAlign="right">{item.quantity}</Td> {/* Right-align Quantity cell */}
                      </Tr>
                    ))}
                  </Tbody>
                </Table>
              </Box>
            </Box>
          </VStack>
        </Grid>
      </Box>
    </ChakraProvider >
  );
}

export default App;
