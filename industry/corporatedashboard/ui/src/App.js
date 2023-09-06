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
  { id: 1, branch: "Stanford", item_type: "Paper", quantity: 500 },
  { id: 2, branch: "Stanford", item_type: "Printers", quantity: 10 },
  { id: 3, branch: "Stanford", item_type: "Ink", quantity: 20 },
];

function App() {
  const [inventoryData, setInventoryData] = useState([]);

  const fetchData = () => {
    // Fetch data from your API endpoint
    fetch("/inventory")
      .then((response) => response.json())
      .then((data) => setInventoryData(data))
      .catch((error) => {
        setInventoryData(backupData);
        console.error("Error fetching data:", error);
      });
  };

  const rundown = () => {
    fetch("/rundown")
      .then((response) => console.dir(response))
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
              <Button color={WASMCLOUD_COLOR} onClick={rundown} mb={4}>Request Rundown</Button>
              <Box overflowX="auto" textAlign="center">
                <Table variant="simple" width="100%" maxWidth="50vw" mx="auto">
                  <Thead>
                    <Tr>
                      <Th>Branch</Th>
                      <Th>Item</Th>
                      <Th textAlign="right">Quantity</Th> {/* Right-align Quantity column */}
                    </Tr>
                  </Thead>
                  <Tbody>
                    {inventoryData.map((item) => (
                      <Tr key={item.id}>
                        {/* TODO: No hardcode */}
                        <Td>{item.branch || "Stanford"}</Td>
                        <Td>{item.item_type}</Td>
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
